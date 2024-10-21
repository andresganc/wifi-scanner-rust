// use wifiscanner::Wifi;

extern crate wifiscanner;

fn main() {
    match wifiscanner::scan() {
        Ok(networks) => {
            for network in networks {
                println!("SSID: {}, Signal: {}", network.ssid, network.signal_level);
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
