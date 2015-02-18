use libc::{c_char, c_int, c_void, size_t, ssize_t, off_t, socklen_t};

/// Iovec data structure for readv and writev calls.
#[repr(C)]
pub struct iovec {
    pub iov_base: *const c_void,
    pub iov_len: size_t,
}

/// A group of messages with a destination to send to
#[repr(C)]
pub struct msghdr {
    /// address to sendto or recvfrom
    msg_name : *mut c_void,
    /// length of the address
    msg_namelen : socklen_t,

    /// buffers to read from or write to
    msg_iov : *mut iovec,
    /// number of buffers to read from or write to
    msg_iovlen : size_t,

    msg_control : *mut c_void,
    msg_controllen : size_t,

    /// MSG_TRUNC shall be set by recvmsg if MSG_PEEK is not set in @flags and the message is too
    /// long to fit in the supplied buffers
    /// recvmsg ignores this field on input, but populates on output.
    ///
    /// sendmsg (???)
    msg_flags : c_int
}

extern "system" {
    /// Read data from fd into multiple buffers
    pub fn readv (fd: c_int, iov: *mut iovec, iovcnt: c_int) -> ssize_t;
    /// Write data from multiple buffers to fd
    pub fn writev (fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;

    /// Read data from fd into multiple buffers
    pub fn preadv (fd: c_int, iov: *mut iovec, iovcnt: c_int, offset: off_t) -> ssize_t;
    /// Write data from multiple buffers to fd
    pub fn pwritev (fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> ssize_t;

    /// Receive a message from a socket
    pub fn recvmsg (socket: c_int, message: *mut msghdr, flags: c_int) -> ssize_t;

    /// Send a message on a socket using a message structure
    pub fn sendmsg (socket: c_int, message: *const msghdr, flags: c_int) -> ssize_t;

    pub fn openat(fd: c_int, path: *const c_char path, oflag: c_int, ...) -> c_int;
    pub fn renameat(fd: c_int, oldfd: c_int, old: *const c_char, newfd: c_int, new: *const c_char) -> c_int;
}

/// Max length for path names. 4096 should be reasonable safe (OS X uses 1024, Linux uses 4096)
pub const PATH_MAX: usize = 4096;
