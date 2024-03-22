#[tokio::main]
async fn main() {
    let client = cadmium_yellow::Client::default();

    let lines = client.lines().unwrap();

    for line in lines {
        println!("{:?}", line.name);

        for station_code in line.stations {
            println!("  {}", station_code);
        }
    }
}
