#[tokio::main]
async fn main() {
    let client = cadmium_yellow::Client::default();

    let station_names = client.station_names().await.unwrap();

    for (code, name) in station_names {
        println!("{} - {}", code, name);
    }
}
