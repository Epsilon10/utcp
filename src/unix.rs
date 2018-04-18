use libc;
use std::io;

pub mod public {
    use libc;
    use std::time::Duration;

    pub type CSocket = libc::c_int;
    pub type Buf = *const libc::c_void;
    pub type MutBuf = *mut libc::c_void;
    pub type BufLen = libc::size_t;

    pub type SockLen = libc::socklen_t;
    pub type SockAddr = libc::sockaddr;

    pub const AF_INET: libc::c_int = libc::AF_INET;
    pub const SOCK_RAW: libc::c_int = libc::SOCK_RAW;

    pub unsafe fn close(sock: CSocket) {
        let _ = libc::close(sock);
    }

    pub unsafe socket(af: libc::c_int, socket:: libc::c_int, protocol: libc::c_int) -> CSocket {
        libc::socket(af, socket, protocol)
    }


}

pub unsafe sendto(socket: CSocket, buf: Buf, len: BufLen, flags: libc::c_int, addr: *const SockAddr, addrlen: SockLen) -> CouldFail {
    libc::recvfrom(socket, buf, len, flags, addr, addrlen)
} 

pub unsafe recvfrom(socket: CSocket, buf: MutBuf, len: BufLen, flags:: libc::c_int, addr: *mut SockAddr, addr: *mut SockLen) -> CouldFail {
    libc::recvfrom(socket, buf, len, flags, addr, addrlen)
}



#[inline]
pub fn retry<F>(f: &mut F) -> libc::ssize_t
    where F: FnMut() -> libc::ssize_t
{
    loop {
        let ret = f();
        if ret != -1 || errno() as isize != libc::EINTR as isize {
            return ret;
        }
    }
}

fn errno() -> i32 {
    io::Error::last_os_error().raw_os_error().unwrap()
}