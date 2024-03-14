#[tokio::main]
async fn main() {
    let client = cadmium_yellow::Client::default();

    let mut stations = client.stations().await;
    stations.sort();

    for station in stations {
        println!("{:#?}", station);
    }
}
