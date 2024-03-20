use reqwest::header::HeaderMap;
use std::collections::HashMap;

const API_URL_BASE: &str = "https://metro-rti.nexus.org.uk/api";
const USER_AGENT: &str = "okhttp/3.12.1";

lazy_static::lazy_static! {
    static ref STATIONS_URL: String = format!("{API_URL_BASE}/stations");
    static ref PLATFORMS_URL: String = format!("{API_URL_BASE}/stations/platforms");
}

fn times_url(station: &str, platform: i64) -> String {
    format!("{API_URL_BASE}/times/{station}/{platform}")
}

/// The source of data returned by the client.
///
/// Only applicable to certain calls.
pub enum DataSource {
    /// Use data baked into the client library at build time.
    Baked,

    /// Request data from the API.
    Api,
}

pub struct Client {
    client: reqwest::Client,
    source: DataSource,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(DataSource::Baked)
    }
}

impl Client {
    pub fn new(source: DataSource) -> Self {
        Self {
            client: reqwest::Client::new(),
            source,
        }
    }

    pub fn set_source(&mut self, source: DataSource) {
        self.source = source;
    }

    fn headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::USER_AGENT, USER_AGENT.parse().unwrap());
        headers
    }

    /// Get a list of all station names and codes.
    pub async fn station_names(&self) -> HashMap<String, String> {
        match self.source {
            DataSource::Baked => {
                serde_json::from_str(include_str!(env!("STATION_NAMES_JSON_FILE"))).unwrap()
            }
            DataSource::Api => self
                .client
                .get(STATIONS_URL.clone())
                .headers(Self::headers())
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap(),
        }
    }

    /// Get a list of platforms at each station.
    pub async fn platforms(&self) -> HashMap<String, Vec<crate::Platform>> {
        match self.source {
            DataSource::Baked => {
                serde_json::from_str(include_str!(env!("PLATFORMS_JSON_FILE"))).unwrap()
            }
            DataSource::Api => self
                .client
                .get(PLATFORMS_URL.clone())
                .headers(Self::headers())
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap(),
        }
    }

    /// Get a list of all stations and platforms at those stations, complete with as much
    /// information as the API provides.
    pub async fn stations(&self) -> Vec<crate::Station> {
        let station_names = self.station_names().await;
        let mut platforms = self.platforms().await;

        assert!(station_names.len() == platforms.len());

        station_names
            .into_iter()
            .map(|(code, name)| crate::Station {
                platforms: platforms.remove(&code).unwrap(),
                code,
                name,
            })
            .collect()
    }

    /// Get the next trains to arrive at a given platform at a given station.
    pub async fn trains(
        &self,
        station: &crate::Station,
        platform: &crate::Platform,
    ) -> Vec<crate::Train> {
        let url = times_url(&station.code, platform.number);

        self.client
            .get(url)
            .headers(Self::headers())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap()
    }
}
