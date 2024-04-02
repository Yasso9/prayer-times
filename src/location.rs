use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

fn public_ip() -> Option<String> {
    let mut ip = None;
    // List all of the machine's network interfaces
    for iface in get_if_addrs::get_if_addrs().ok()? {
        // println!("IP Found : {:#?}", iface.ip());
        if iface.is_loopback() {
            continue;
        }
        let ip_addr = iface.ip().to_string();
        if ip_addr.starts_with("192.168") {
            continue;
        }
        // println!("IP : {:#?}", iface.ip());
        ip = Some(ip_addr);
        // println!("IP : {:#?}", iface.type_id());
        // println!("{:#?}", iface.is_loopback());
    }

    ip
}

pub fn current_location(print: bool) -> Option<Location> {
    let info = geolocation::find(public_ip()?.as_str()).ok()?;
    let lat: Result<f64, _> = info.latitude.parse();
    let lon: Result<f64, _> = info.longitude.parse();

    if print {
        println!("Location automatically detected:");
        println!("Latitude: {}", info.latitude);
        println!("Longitude: {}", info.longitude);
        println!("City: {}", info.city);
        println!("Country: {}", info.country);
        println!("\n");
    }

    Some(Location {
        lat: lat.ok()?,
        lon: lon.ok()?,
    })
}
