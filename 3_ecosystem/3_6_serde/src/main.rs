use humantime_serde;
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use std::{fs, time::Duration};
use toml;

fn main() {
    const PATH: &str = "./request.json";
    let data = fs::read_to_string(PATH).expect("Couldn't read the file");
    let request: Request = serde_json::from_str(&data).expect("Couldn't parse JSON");

    println!("YAML");
    println!("{}", serde_yaml::to_string(&request).unwrap());
    println!("TOML");
    println!("{}", toml::to_string(&request).unwrap());
}

#[derive(Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    result_type: String,
    stream: Stream,
    gifts: Vec<Gift>,
    #[serde(rename = "debug")]
    debug_info: DebugInfo,
}

#[derive(Serialize, Deserialize)]
struct Stream {
    user_id: String,
    is_private: bool,
    settings: i32, // looks like a bitmask of some sort
    shard_url: String,
    public_tariff: Option<PublicTariff>,
    private_tariff: Option<PrivateTariff>,
}

#[derive(Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct PrivateTariff {
    #[serde(rename = "client_price")]
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Serialize, Deserialize)]
// It's a debug info and it's annoying to convert to better formats, so it stays
// as it were before.
struct DebugInfo {
    duration: String,
    at: String,
}

// I'm not even sure what to test here...
