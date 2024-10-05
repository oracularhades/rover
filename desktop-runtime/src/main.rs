pub mod globals {
    pub mod credential;
    pub mod macos;
    pub mod object;
    pub mod websocket;
}

pub mod structs {
    pub mod credential;
}

use std::time::{SystemTime, UNIX_EPOCH};

use globals::credential::{credentials_get, credentials_set};
use globals::macos::{retrieve_password, store_password};
use globals::websocket::websocket_event_builder;
use hades_auth::{static_auth_sign, Sign};
use serde_json::{json, Value};
use sysinfo::{
    Components, Disks, Networks, Process, System
};

use once_cell::sync::Lazy;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, watch};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, tungstenite::protocol::CloseFrame};
use futures_util::{StreamExt, SinkExt};

use crate::globals::websocket::websocket_connect_feed_channel;

pub static AUTH_DEFAULT_DATA: Lazy<serde_json::Value> = Lazy::new(|| {
    let credentials = credentials_get();
    
    return json!({
        "device_id": credentials.device_id
    });
});

pub static CHANNEL: Lazy<(Mutex<mpsc::UnboundedSender<String>>, Mutex<mpsc::UnboundedReceiver<String>>)> = Lazy::new(|| {
    let (tx, rx) = mpsc::unbounded_channel(); // Use tokio's mpsc::channel instead of std::sync
    (Mutex::new(tx), Mutex::new(rx))
});

pub static ACTIVE_PID: Lazy<Mutex<Vec<u32>>> = Lazy::new(|| Mutex::new(Vec::new()));

async fn send_processes(processes: Vec<Value>) {
    let mut tx = CHANNEL.0.lock().await;

    let data = json!({
        "processes": processes
    });

    let additional_metadata = websocket_event_builder("process", &data, &(*AUTH_DEFAULT_DATA).clone(), credentials_get()).await;
    // let static_auth = Sign((*AUTH_DEFAULT_DATA).clone(), Some(&serde_json::to_string(&additional_metadata).unwrap()), &credentials.private_key, None).await.expect("Failed to generate static_auth");
    tx.send(serde_json::to_string(&additional_metadata).unwrap()).unwrap();
}

async fn thing() {
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();

    let mut num = 0;
    loop {
        // First we update all information of our `System` struct.
        num += 1;
        // println!("REFRESH {}", num);
        sys.refresh_all();

        let mut processes: Vec<Value> = Vec::new();

        // println!("=> system:");
        // // Memory
        // println!("total memory: {} bytes", sys.total_memory());
        // println!("used memory : {} bytes", sys.used_memory());
        // println!("total swap  : {} bytes", sys.total_swap());
        // println!("used swap   : {} bytes", sys.used_swap());

        // // Display system information:
        // println!("System name:             {:?}", System::name());
        // println!("System kernel version:   {:?}", System::kernel_version());
        // println!("System OS version:       {:?}", System::os_version());
        // println!("System host name:        {:?}", System::host_name());

        // // CPU
        // println!("NB CPUs: {}", sys.cpus().len());

        // Display processes ID, name na disk usage:
        for (pid, process) in sys.processes() {
            let mut pids = ACTIVE_PID.lock().await;
            if (pids.contains(&pid.as_u32()) == false) {
                pids.push(pid.as_u32());

                let process_data = json!({
                    "pid": process.pid().as_u32(),
                    "parent": process.parent().map(|d|d.as_u32()),
                    "name": process.name().to_str(),
                    "path": process.exe(),
                    "current_working_directory": process.cwd(), // need to log in Rover when a working directory changes.
                    "status": process.status().to_string(),
                    "run_time": process.run_time(),
                    "start_time": process.start_time()
                });
                processes.push(process_data);
            }
        }
        if (processes.len() > 0) {
            tokio::spawn(async move {
                send_processes(processes).await;
            });
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    credentials_set("https://rover.internal.motionfans.com".into(), "YPQQKMPRUXQZLMAFWOOQ1726104804819".into(), "MIHuAgEAMBAGByqGSM49AgEGBSuBBAAjBIHWMIHTAgEBBEIATLEtb8HUJ2FzMgH+YSijCZ78Y/iUydG/kVM5PIGrWoj//ZYZtf5jqQiM3CILAYbTjijVlJXUZjmcj5HEGdU6XhChgYkDgYYABAHc2cY50CljTxIoHYrIu+sBaPdD/fUuh+aqNe4crcjWU9OU4DpoXIojHCrCvNlkTv7tsSoiHdw2ck8klAoGQhdYNQFE0FqXn87zi1UgPprTujqF5OovEja7zvF+HLd1g0X5Gaqjny6S/cCUZyvHx4SaBKFRqk2Jva4wT6rwl6BioTIOZQ==".into());
    println!("CREDENTIAL: {:?}", credentials_get());
    tokio::spawn(async move {
        websocket_connect_feed_channel("ws://127.0.0.1:8080").await;
    });
    
    thing().await;

    // // Disk
    // println!("=> disks:");
    // let disks = Disks::new_with_refreshed_list();
    // for disk in &disks {
    //     println!("{disk:?}");
    // }

    // // Network
    // let networks = Networks::new_with_refreshed_list();
    // println!("=> networks:");
    // for (interface_name, data) in &networks {
    //     println!(
    //         "{interface_name}: {} B (down) / {} B (up) || {:?} || {:?}",
    //         data.total_received(),
    //         data.total_transmitted(),
    //         data.ip_networks(),
    //         data.received()
    //     );
    //     // If you want the amount of data received/transmitted since last call
    //     // to `Networks::refresh`, use `received`/`transmitted`.
    // }

    // // System temps
    // let components = Components::new_with_refreshed_list();
    // println!("=> components:");
    // for component in &components {
    //     println!("{component:?}");
    // }
}