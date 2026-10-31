mod parse;
mod rcv_pkt;
mod update_devices;

use std::sync::Arc;
use std::sync::atomic::Ordering;
use dashmap;
use dashmap::DashMap;
use pcap::Device;
use tokio;
use update_devices::Devicetask;

fn get_devices() -> Result<Vec<Device>, pcap::Error> {
    let devices = Device::list()?;
    Ok(devices)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let  device_map:Arc< DashMap<String, Devicetask>> = Arc::new(DashMap::new());
    tokio::select! {
         result = update_devices::update_devices( device_map.clone()) => {
   
        }

        result = tokio::signal::ctrl_c() => {
            println!("Control C exit ... ");
            for task in device_map.iter() {
                task.stop.store(true, Ordering::Relaxed);
             }
        }

    }



    
    Ok(())
}
