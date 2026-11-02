use std::sync::atomic::{AtomicBool, Ordering};
use pcap::{Capture, Device};
use std::sync::Arc;
use crate::parse::parse_pkt;

pub fn rcv_pkt(device: Device,stop : Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut cap = Capture::from_device(device)?
        .promisc(true)
        .immediate_mode(true)
        .timeout(1000)
        .open()?;

    let datalink = (&cap).get_datalink();
    loop {
         if stop.load(Ordering::Relaxed) { break ; }
        match cap.next_packet() {
            Ok(packet) => {

              let _ =  parse_pkt(packet,datalink);

            }
            Err(pcap::Error::TimeoutExpired) => {
                continue;
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}
