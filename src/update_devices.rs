use std::collections::HashSet;
use crate::{get_devices, rcv_pkt};
use dashmap::{DashMap, Entry};
use tokio::task::JoinHandle;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::atomic::Ordering;


pub struct Devicetask {
   pub handle: JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
   pub  stop: Arc<AtomicBool>,
}

impl Devicetask {
    pub fn new(
        handle: JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
        stop: Arc<AtomicBool>,
    ) -> Self {

        Self { handle, stop }
    }
}
pub async fn update_devices(
    device_map:  Arc<DashMap<String, Devicetask>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
    let mut device_set:HashSet<String> = HashSet::new();
    loop {

        let devices = get_devices()?;
        device_set.clear();
        for device in &devices {

                device_set.insert(device.name.clone());
        }
        for device in devices {
            let id = device.name.clone();
            match device_map.entry(id) {
                Entry::Vacant(entry) => {

                    //insert & spawn

                    let stop =Arc::new(AtomicBool::new(false));
                    let stop_clone = stop.clone();
                    let handler = tokio::task::spawn_blocking(
                        move ||
                            rcv_pkt::rcv_pkt(device,stop)
                    );

                    entry.insert(Devicetask::new(handler,stop_clone));
                }
                Entry::Occupied(_) => {
                    //eat five star
                }
            }
        }

        device_map.retain(|device_name, task| {
            if device_set.contains(device_name) {
                true
            } else {
                task.stop.store(true, Ordering::Relaxed);
                false
            }
        });
        interval.tick().await;
    }
}
