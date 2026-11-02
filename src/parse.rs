use std::error;
use std::net::IpAddr;

use etherparse::{NetSlice, PacketHeaders, TransportSlice};
use etherparse::SlicedPacket;
use pcap::{Linktype, Packet};
use log::error;



pub fn parse_pkt(packet: Packet, linktype: Linktype) ->Result<(),Box<dyn std::error::Error> > {
    match linktype {
        Linktype(0) => {
            let bytes = packet.data;

            if bytes.len() < 4 {
                return Ok(());
            }
            let remaining = &bytes[4..];
            let sliced =SlicedPacket::from_ip(remaining)?;


            match sliced.net {
                Some(etherparse::NetSlice::Ipv4(header)) => {
                    let src = header.header().source_addr();
                    let dst = header.header().destination_addr();

                    println!("Source IP: {}", src);
                    println!("Destination IP: {}", dst);
                }

                Some(etherparse::NetSlice::Ipv6(header)) => {
                    let src = header.header().source_addr();
                    let dst = header.header().destination_addr();

                    println!("Source IP: {}", src);
                    println!("Destination IP: {}", dst);
                }

                None => {}
                _ => {}
            }

            match sliced.transport {
                Some(etherparse::TransportSlice::Tcp(tcp)) => {
                    println!("TCP");
                    println!("Source port: {}", tcp.source_port());
                    println!("Destination port: {}", tcp.destination_port());
                }

                Some(etherparse::TransportSlice::Udp(udp)) => {
                    println!("UDP");
                    println!("Source port: {}", udp.source_port());
                    println!("Destination port: {}", udp.destination_port());
                }

                Some(etherparse::TransportSlice::Icmpv4(_)) => {
                    println!("ICMPv4");
                }

                Some(etherparse::TransportSlice::Icmpv6(_)) => {
                    println!("ICMPv6");
                }

                None => {},
                Some(TransportSlice::Igmp(_)) => todo!(),

            }




            Ok(())
        }
        Linktype(1) => match PacketHeaders::from_ethernet_slice(packet.data) {
            Ok(p) => { Ok(())}
            Err(e) => {Ok(())}
        },
        _ => {
            println!("Linktype is not supported  :) ");
            Ok(())
        }
    }
}
