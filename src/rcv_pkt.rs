use std::sync::atomic::{AtomicBool, Ordering};
use pcap::{Capture, Device};
use std::sync::Arc;
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
                println!("Received packet! {:?} {:?}", packet.header, datalink);
                //parse_pkt will be called here
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
