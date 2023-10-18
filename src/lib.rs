#![no_std]
#![feature(c_variadic)]

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
#[cfg(all(target_os = "vita", feature = "socketpair"))]
pub unsafe extern "C" fn socketpair(
    _domain: libc::c_int,
    r#type: libc::c_int,
    protocol: libc::c_int,
    socket_vector: *mut libc::c_int,
) -> libc::c_int {
    let mut server_addr: libc::sockaddr_in = core::mem::zeroed();
    let mut addr_len: libc::socklen_t =
        core::mem::size_of::<libc::sockaddr_in>() as libc::socklen_t;

    let listener = libc::socket(libc::AF_INET, r#type, protocol);
    if listener == -1 {
        return -1;
    }

    server_addr.sin_family = libc::AF_INET as libc::sa_family_t;
    server_addr.sin_port = 0;
    server_addr.sin_addr.s_addr = libc::INADDR_LOOPBACK.to_be();

    if libc::bind(listener, &mut server_addr as *mut _ as *mut _, addr_len) == -1 {
        with_errno(|| {
            libc::close(listener);
        });
        return -1;
    }

    if libc::listen(listener, 1) == -1 {
        with_errno(|| {
            libc::close(listener);
        });
        return -1;
    }

    if libc::getsockname(
        listener,
        &mut server_addr as *mut _ as *mut _,
        &mut addr_len,
    ) == -1
    {
        with_errno(|| {
            libc::close(listener);
        });
        return -1;
    }

    let client_socket: libc::c_int = libc::socket(libc::AF_INET, r#type, protocol);
    if client_socket == -1 {
        with_errno(|| {
            libc::close(listener);
        });
        return -1;
    }

    if libc::connect(
        client_socket,
        &mut server_addr as *mut _ as *mut _,
        addr_len,
    ) == -1
    {
        with_errno(|| {
            libc::close(client_socket);
            libc::close(listener);
        });
        return -1;
    }

    let peer_socket: libc::c_int = libc::accept(
        listener,
        &mut server_addr as *mut _ as *mut _,
        &mut addr_len,
    );
    if peer_socket == -1 {
        with_errno(|| {
            libc::close(client_socket);
            libc::close(listener);
        });
        return -1;
    }

    socket_vector.offset(0).write(peer_socket);
    socket_vector.offset(1).write(client_socket);

    libc::close(listener);

    0
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
#[cfg(all(target_os = "vita", feature = "pipe2"))]
pub unsafe extern "C" fn pipe2(pipefd: &mut [libc::c_int; 2], flags: libc::c_int) -> libc::c_int {
    #[cfg(not(feature = "socketpair"))]
    use libc::socketpair;

    if socketpair(libc::AF_INET, libc::SOCK_STREAM, 0, pipefd.as_mut_ptr()) == -1 {
        return -1;
    }

    let pipefd = *pipefd;
    for fd in pipefd {
        let linger = libc::linger {
            l_onoff: 1,
            l_linger: 0,
        };
        if setsockopt(fd, libc::SOL_SOCKET, libc::SO_LINGER, linger) == -1 {
            with_errno(|| {
                libc::close(pipefd[0]);
                libc::close(pipefd[1]);
            });
            return -1;
        }
    }

    if flags & libc::O_NONBLOCK != 0 {
        for fd in pipefd {
            if setsockopt(fd, libc::SOL_SOCKET, libc::SO_NONBLOCK, 1) == -1 {
                with_errno(|| {
                    libc::close(pipefd[0]);
                    libc::close(pipefd[1]);
                });
                return -1;
            }
        }
    }

    0
}

#[cfg(all(
    target_os = "vita",
    any(feature = "pipe2", feature = "socketpair", feature = "fcntl")
))]
extern "C" {
    #[cfg_attr(target_os = "vita", link_name = "__errno")]
    fn errno_location() -> *mut libc::c_int;
}

#[cfg(all(target_os = "vita", any(feature = "pipe2", feature = "socketpair")))]
unsafe fn with_errno(mut f: impl FnMut()) {
    let errno = *errno_location();
    f();
    *errno_location() = errno as libc::c_int;
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
#[cfg(all(target_os = "vita", feature = "fcntl"))]
pub unsafe extern "C" fn fcntl(fd: libc::c_int, cmd: libc::c_int, mut args: ...) -> libc::c_int {
    let mut arg: libc::c_int = 0;

    if cmd == libc::F_SETFL {
        arg = args.arg::<libc::c_int>();
    }

    match cmd {
        libc::F_GETFD => 0,
        libc::F_GETFL => {
            let mut val: libc::c_int = 0;

            let res = getsockopt(fd, libc::SOL_SOCKET, libc::SO_NONBLOCK, &mut val);
            if res == -1 {
                return -1;
            }

            match val {
                0 => 0,
                _ => libc::O_NONBLOCK,
            }
        }
        libc::F_SETFL => {
            let val = (arg & libc::O_NONBLOCK) != 0;
            setsockopt(fd, libc::SOL_SOCKET, libc::SO_NONBLOCK, val as libc::c_int)
        }
        _ => {
            *errno_location() = libc::ENOTSUP;
            -1
        }
    }
}

#[cfg(all(target_os = "vita", any(feature = "pipe2", feature = "fcntl")))]
unsafe fn setsockopt<T>(
    fd: libc::c_int,
    level: libc::c_int,
    name: libc::c_int,
    val: T,
) -> libc::c_int {
    libc::setsockopt(
        fd,
        level,
        name,
        &val as *const _ as *const _,
        core::mem::size_of::<T>() as libc::socklen_t,
    )
}

#[cfg(all(target_os = "vita", feature = "fcntl"))]
unsafe fn getsockopt<T>(
    fd: libc::c_int,
    level: libc::c_int,
    name: libc::c_int,
    val: &mut T,
) -> libc::c_int {
    let mut len: libc::socklen_t = core::mem::size_of::<T>() as libc::socklen_t;
    libc::getsockopt(
        fd,
        level,
        name,
        val as *mut _ as *mut _,
        &mut len as *mut _ as *mut _,
    )
}
