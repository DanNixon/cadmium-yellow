#[tokio::main]
async fn main() {
    let client = cadmium_yellow::Client::new(cadmium_yellow::DataSource::Api);

    let mut stations = client.stations().await;
    stations.sort();

    for station in stations {
        println!("{:#?}", station);
    }
}
