use pcap::{Capture, Device};

pub fn rcv_pkt(device: Device) ->Result<(),Box<dyn std::error::Error +Send +Sync >>{
    
    let mut cap = Capture::from_device(device)?
        .promisc(true)
        .immediate_mode(true)
        .timeout(1000)
        .open()?;
    let datalink = (&cap).get_datalink();
    loop {
        match cap.next_packet() {
            Ok(packet) => {

                println!("Received packet! {:?} {:?}", packet.header,datalink);
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