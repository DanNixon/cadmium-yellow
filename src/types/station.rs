use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Station {
    /// The signposted name of the station (unless it's Monument)
    pub name: String,

    /// The unique identifier for the station
    pub code: String,

    /// The platforms available at the station
    pub platforms: Vec<crate::Platform>,
}

impl Ord for Station {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Station {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
