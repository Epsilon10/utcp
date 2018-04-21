extern crate libc;
use unix_socket::*;

use std::io;
use std::sync::Arc;
use std::net::{self, IpAddr};

// Reps transport layer protocol
#[derive(Clone, Copy)]
pub enum TransportProtocol {
    IpV4(IpNextHeaderProtocol)
}

#[derive(Clone, Copy)]
pub enum TransportChannelType {
    Layer4(TransportProtocol),
    Layer3(IpNextHeaderProtocol),
}

// Struct for sending at the transport layer
pub struct TransportSender {
    socket: Arc<unix_socket::FileDescriptor>,
    _channel_type: TransportChannelType
}

// Struct for recieving at the transport layer
pub struct TransportReciever {
    socket: Arc<unix_socket::FileDescriptor>,
    buffer: Vec<u8>,
    channel_type: TransportChannelType
}

pub fn transport_channel(buffer_size: usize, channel_type: TransportChannelType) -> io::Result<(TransportSender, TransportReciever)> {
    use std::net;
    let socket = unsafe {
        match channel_type {
            Layer4(IpV4(IpNextHeaderProtocol(proto))) |
            Layer3(IpNextHeaderProtocol(proto)) => {
                unix_socket::socket(unix::AF_INET, unix::SOCK_RAW, proto as libc::c_int)
            }
        }
    };

    if socket == pnet_sys::INVALID_SOCKET {
        return Err(Error::last_os_error());
    }


}

impl TransportSender {
    fn send<T>(&mut self, packet: T, dst: IpAddr) -> Io::Result<usize> {
        let mut caddr = unsafe { mem::zeroed() };
        let sockaddr = match dst {
            IpAddr::V4(ip_addr) => net::SocketAddr::V4(net::SocketAddrV4::new(ip_addr, 0)),

        };
        let slen = pnet_sys::addr_to_sockaddr(sockaddr, &mut caddr);
        let caddr_ptr = (&caddr as *const unix::SocketAddrStorage) as *const unix::SocketAddr;

        unix_socket::send_to(self.socket.fd, )


    }

    #[inline]
    pub fn send_to<T: Packet>(&mut self, pakcet: T, dst: IpAddr) -> io::Result<usize> {
        self.send_to_impl(packet, dst);
    }

    fn send_to_impl<T: Packet>(&mut self, packet: T, dst: IpAddr) -> io::Result<usize> {
        self.send(packet, dst);
    }

    

    

     
}