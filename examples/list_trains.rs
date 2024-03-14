#[tokio::main]
async fn main() {
    let client = cadmium_yellow::Client::default();

    let stations = client.stations().await;

    let station = stations.iter().find(|s| s.name == "Haymarket").unwrap();
    println!("{:#?}", station);

    let platform = station.platforms.iter().find(|p| p.number == 1).unwrap();
    println!("{:#?}", platform);

    let trains = client.trains(station, platform).await;

    for train in trains {
        println!("{:#?}", train);
    }
}
