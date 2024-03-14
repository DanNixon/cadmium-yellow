use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, PartialOrd, Deserialize)]
pub struct Platform {
    /// Signposted platform number
    #[serde(rename = "platformNumber")]
    pub number: i64,

    /// A machine friendly description of what direction trains on the platform depart in.
    pub direction: String,

    /// A user friendly description of what direction trains on the platform depart in.
    #[serde(rename = "helperText")]
    pub help_text: String,
}
