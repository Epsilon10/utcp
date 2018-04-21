pub struct Tcp {
    source_port: u16,
    dest_port: u16,
    seq_no: u32,
    ack_no: u32,
    offset_reserved_control: u16,
    window: u16,
    checksum: u16,
    urgent_pointer: u16
    padding: u8,
    data: &mut [u8]
}

pub struct IpV4Header {
    version_ihl = u8,
    dscp_ecn = u8,
    total_len: u16,
    identification: u16, 
    flags_offset: u16,
    liftime: u8,
    protocol: u8,
    checksum: u8,
    source_addr: u32, // source ip addr
    dest_addr: u32,
    options: Option<u32>,
    padding: Option<u32>
    

}

pub struct Packet<T> {
    header: IpV4Header,
    packet:  T
}

impl<T> Packet<T> {
    pub fn new<T, H>(packet: T, header: H) -> Packet {
        Packet {
            header,
            packet
        }
    }

    pub fn window(&self) -> u16{

    }
}






