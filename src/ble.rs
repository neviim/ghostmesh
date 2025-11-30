use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use tracing::{info, error, warn};
use uuid::Uuid;

// GhostMesh Service UUID: 12345678-1234-1234-1234-1234567890ab (Example)
const _GHOSTMESH_UUID: Uuid = Uuid::from_u128(0x12345678_1234_1234_1234_1234567890ab);

pub async fn run_ble_service() -> Result<(), Box<dyn Error>> {
    info!("Starting BLE Service...");

    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    
    if adapters.is_empty() {
        warn!("No Bluetooth adapters found. BLE features will be disabled.");
        return Ok(());
    }

    let adapter = adapters.into_iter().nth(0).unwrap();
    info!("Using Bluetooth Adapter: {:?}", adapter.adapter_info().await?);

    // Start Scanning
    if let Err(e) = adapter.start_scan(ScanFilter::default()).await {
        error!("Failed to start BLE scan: {:?}", e);
        return Ok(()); // Don't crash the node
    }

    info!("BLE Scanning started...");

    let mut events = adapter.events().await?;

    // Advertising is complex and platform-dependent. 
    // For this prototype, we will focus on SCANNING to detect presence.
    // Implementing advertising requires BlueZ DBus API interaction which btleplug 
    // supports but can be flaky without root/configuration.
    
    // Simple loop to log discovered devices
    loop {
        tokio::select! {
            Some(event) = events.next() => {
                match event {
                    btleplug::api::CentralEvent::DeviceDiscovered(_id) => {
                        // info!("BLE Device Discovered: {:?}", id);
                        // In a real implementation, we would connect and check for GhostMesh service
                    }
                    btleplug::api::CentralEvent::DeviceUpdated(id) => {
                        // Check if it has our UUID (if advertised)
                        if let Ok(peripheral) = adapter.peripheral(&id).await {
                            if let Ok(Some(props)) = peripheral.properties().await {
                                if let Some(local_name) = props.local_name {
                                    if local_name.contains("GhostMesh") {
                                        info!("Found GhostMesh Peer via BLE: {} ({:?})", local_name, id);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ = time::sleep(Duration::from_secs(60)) => {
                // Heartbeat
            }
        }
    }
}
