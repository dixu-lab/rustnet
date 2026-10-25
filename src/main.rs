mod rcv_pkt;
mod parse;

use pcap::{ Device};
use tokio;
fn get_devices()->Result<Vec<Device>,pcap::Error>{
    let devices = Device::list()?;
    Ok(devices)

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let devices =   get_devices()?;

    // for device in devices {
    //    let _handler =
    //                 tokio::task::spawn_blocking(
    //                     move || rcv_pkt::rcv_pkt(device)
    //                 );
    // }

    let handlers = devices.into_iter().map(|device| {
        tokio::task::spawn_blocking(move || {
            rcv_pkt::rcv_pkt(device)
        })
    }
    );

    tokio::signal::ctrl_c().await?;
    println!("exiting with ctrl c");
    Ok(())
}
