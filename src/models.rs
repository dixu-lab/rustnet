use std::net::IpAddr;

pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    ICMPv6,
    Other(u8),
}
pub struct ParsedPacket {
    src_ip: Option<IpAddr>,
    dst_ip: Option<IpAddr>,
    src_port: Option<u16>,
    dst_port: Option<u16>,
    protocol: Option<Protocol>,
    packet_size: usize,
}
impl ParsedPacket{
    pub fn new(src_ip:Option<IpAddr>,dst_ip:Option<IpAddr>,src_port:Option<u16>,dst_port:Option<u16>,protocol:Option< Protocol>,packet_size:usize)->Self
    {
        Self{
            src_ip,
            dst_ip,
            src_port,
            dst_port,
            protocol,
            packet_size
        }
    }
}