use etherparse::PacketHeaders;
use pcap::{Linktype, Packet};

pub fn parse_pkt(packet: Packet, linktype: Linktype) {
    match linktype {
        Linktype(0) => {
            let bytes = packet.data;

            if bytes.len() < 4 {
                return;
            }
            let remaining = &bytes[4..];
        }
        Linktype(1) => match PacketHeaders::from_ethernet_slice(packet.data) {
            Ok(p) => {}
            Err(e) => {}
        },
        _ => {
            println!("Linktype is not supported  :) ");
        }
    }
}
