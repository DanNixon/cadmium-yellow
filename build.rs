use reqwest::{blocking::Client, header::HeaderMap};
use std::{fs::File, io::Write};

const API_URL_BASE: &str = "https://metro-rti.nexus.org.uk/api";
const USER_AGENT: &str = "okhttp/3.12.1";

lazy_static::lazy_static! {
    static ref STATIONS_URL: String = format!("{API_URL_BASE}/stations");
    static ref PLATFORMS_URL: String = format!("{API_URL_BASE}/stations/platforms");
}

fn get_json_file(client: &Client, headers: &HeaderMap, url: &str, path: &str) {
    if !std::path::Path::new(path).is_file() {
        let response = client.get(url).headers(headers.clone()).send().unwrap();
        let content = response.text().unwrap();

        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}

fn main() {
    let client = Client::new();

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let headers = {
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, USER_AGENT.parse().unwrap());
        headers
    };

    let filename = format!("{out_dir}/station_names.json");
    get_json_file(&client, &headers, &STATIONS_URL, &filename);
    println!("cargo:rustc-env=STATION_NAMES_JSON_FILE={filename}");

    let filename = format!("{out_dir}/platforms.json");
    get_json_file(&client, &headers, &PLATFORMS_URL, &filename);
    println!("cargo:rustc-env=PLATFORMS_JSON_FILE={filename}");
}
