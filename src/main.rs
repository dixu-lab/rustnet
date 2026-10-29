mod rcv_pkt;
mod parse;

use tokio::task::JoinHandle;
use pcap::{ Device};
use tokio;
use dashmap;
use dashmap::DashMap;
use dashmap::mapref::entry::Entry;

fn get_devices()->Result<Vec<Device>,pcap::Error>{

    let devices = Device::list()?;
    Ok(devices)

}
async fn update_devices(device_map:& mut DashMap<String,  JoinHandle<  Result<(),Box <dyn std::error::Error + Send + Sync>  >  >  >)-> Result<(), Box<dyn std::error::Error>> {
    let mut interval = tokio::time::interval(
        tokio::time::Duration::from_secs(1)
    );
    loop{
        let devices =   get_devices()?;
        for device in devices {
            let id = device.name.clone();
            match device_map.entry(id) {
                Entry::Vacant(entry) => {
                        //insert & spawn
                   let handler = tokio::task::spawn_blocking(move || {
                        rcv_pkt::rcv_pkt(device)
                    });
                    entry.insert(handler);
                }
                Entry::Occupied(_) => {
                    //eat five star
                }
            }
        }

        interval.tick().await;
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


   let mut device_map:DashMap<String,   JoinHandle<  Result<(),Box <dyn std::error::Error +Send+Sync>  >  >  >  = DashMap::new();
    update_devices(&mut device_map).await?;



    tokio::signal::ctrl_c().await?;
    println!("exiting with ctrl c");
    Ok(())
}
