#![no_std]
#![feature(c_variadic)]

#[no_mangle]
#[cfg(target_os = "vita")]
pub unsafe extern "C" fn getppid() -> libc::pid_t {
    1
}

#[no_mangle]
#[cfg(target_os = "vita")]
pub unsafe extern "C" fn ioctl(_fd: libc::c_int, _request: libc::c_ulong, ...) -> libc::c_int {
    unimplemented!("ioctl is unsupported");
}

#[no_mangle]
#[cfg(target_os = "vita")]
pub unsafe extern "C" fn symlink(
    _path1: *const libc::c_char,
    _path2: *const libc::c_char,
) -> libc::c_int {
    unimplemented!("symlink is unsupported")
}

#[no_mangle]
#[cfg(target_os = "vita")]
pub unsafe extern "C" fn readlink(
    _path: *const libc::c_char,
    _buf: *mut libc::c_char,
    _bufsz: libc::size_t,
) -> libc::ssize_t {
    unimplemented!("readlink is unsupported")
}

#[no_mangle]
#[cfg(target_os = "vita")]
pub unsafe extern "C" fn linkat(
    _olddirfd: libc::c_int,
    _oldpath: *const libc::c_char,
    _newdirfd: libc::c_int,
    _newpath: *const libc::c_char,
    _flags: libc::c_int,
) -> libc::c_int {
    unimplemented!("linkat is unsupported")
}

#[no_mangle]
#[cfg(target_os = "vita")]
pub unsafe extern "C" fn fchown(
    _fd: libc::c_int,
    _owner: libc::uid_t,
    _group: libc::gid_t,
) -> libc::c_int {
    unimplemented!("fchown is unsupported")
}

#[no_mangle]
#[cfg(target_os = "vita")]
pub unsafe extern "C" fn lchown(
    _path: *const libc::c_char,
    _uid: libc::uid_t,
    _gid: libc::gid_t,
) -> libc::c_int {
    unimplemented!("lchown is unsupported")
}

#[no_mangle]
#[cfg(target_os = "vita")]
pub unsafe extern "C" fn socketpair(
    _domain: libc::c_int,
    _type_: libc::c_int,
    _protocol: libc::c_int,
    _socket_vector: *mut libc::c_int,
) -> libc::c_int {
    unimplemented!("socketpair is unsupported")
}
