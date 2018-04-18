use libc::CSocket;

use std::io;
use std::mem;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

mod unix;

pub use self::unix::public::*;

pub struct FileDescriptor {
    pub file_desc: CSocket
}

impl Drop for FileDescriptor {
    fn drop(&mut self) {
        unsafe { close(self.fd) }
    }
}

pub fn send_to(socket: CSocket, buffer: &[u8], dst: *const SocketAddr, slen: SockLen) -> io::Result<usize> {
    let send_len = unix::retry(&mut || unsafe {
        unix::sendto(socket, buffer.as_ptr() as Buf, buffer.len() as BufLen, 0, dst, slen)
    });

    if send_len < 0 {
        Err(io::Error::last_os_error())
    }

    else {
        Ok(send_len as usize)
    }

    
} 

pub fn recv_from(socket: CSocket, buffer: &mut [u8], caddr: SocketAddrStorage) -> io::Result<usize>{
    let mut caddrlen = mem::size_of::<SocketAddrStorage>() as SockLen;
    let len = unix::retry(&mut || unsafe {
        unix::recvfrom(socket, buffer.as_ptr() as MutBuf, buffer.len(), 0, caddr as *mut SocketAddr, caddrlen as *mut SockLen)
    });

    if len < 0 {
        Err(io::Error::last_os_error())
    }

    else {
        Ok(len as usize)
    }

}

