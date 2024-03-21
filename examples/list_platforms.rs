#[tokio::main]
async fn main() {
    let client = cadmium_yellow::Client::default();

    let platforms = client.platforms().await.unwrap();

    for (station_code, station_platforms) in platforms {
        println!("{}", station_code);

        for platform in station_platforms {
            println!("  {:?}", platform);
        }
    }
}
