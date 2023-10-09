#![no_std]

#[no_mangle]
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
        libc::close(listener);
        return -1;
    }

    if libc::listen(listener, 1) == -1 {
        libc::close(listener);
        return -1;
    }

    if libc::getsockname(
        listener,
        &mut server_addr as *mut _ as *mut _,
        &mut addr_len,
    ) == -1
    {
        libc::close(listener);
        return -1;
    }

    let client_socket: libc::c_int = libc::socket(libc::AF_INET, r#type, protocol);
    if client_socket == -1 {
        libc::close(listener);
        return -1;
    }

    if libc::connect(
        client_socket,
        &mut server_addr as *mut _ as *mut _,
        addr_len,
    ) == -1
    {
        libc::close(client_socket);
        libc::close(listener);
        return -1;
    }

    let peer_socket: libc::c_int = libc::accept(
        listener,
        &mut server_addr as *mut _ as *mut _,
        &mut addr_len,
    );
    if peer_socket == -1 {
        libc::close(client_socket);
        libc::close(listener);
        return -1;
    }

    socket_vector.offset(0).write(peer_socket);
    socket_vector.offset(1).write(client_socket);

    libc::close(listener);

    0
}

#[no_mangle]
#[cfg(all(target_os = "vita", feature = "pipe2"))]
pub unsafe extern "C" fn pipe2(pipefd: &mut [libc::c_int; 2], flags: libc::c_int) -> libc::c_int {
    #[cfg(not(feature = "socketpair"))]
    use libc::socketpair;

    if flags & libc::O_NONBLOCK != 0 {
        if socketpair(libc::AF_INET, libc::SOCK_STREAM, 0, pipefd.as_mut_ptr()) == -1 {
            return -1;
        }

        let val: libc::c_int = 1;
        if set_nonblocking(pipefd[0], val) == -1 || set_nonblocking(pipefd[1], val) == -1 {
            libc::close(pipefd[0]);
            libc::close(pipefd[1]);
            return -1;
        }

        0
    } else {
        libc::pipe(pipefd.as_mut_ptr())
    }
}

#[cfg(all(target_os = "vita", feature = "pipe2"))]
unsafe fn set_nonblocking(fd: libc::c_int, val: libc::c_int) -> libc::c_int {
    libc::setsockopt(
        fd,
        libc::SOL_SOCKET,
        libc::SO_NONBLOCK,
        &val as *const _ as *const _,
        core::mem::size_of::<libc::c_int>() as u32,
    )
}
