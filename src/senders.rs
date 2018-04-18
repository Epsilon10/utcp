extern crate libc;
use unix_socket::*;

use std::io;
use std::sync::Arc;
use std::net::{self, IpAddr};

// Reps transport layer protocol
#[derive(Clone, Copy)]
pub enum TransportProtocl {
    
}