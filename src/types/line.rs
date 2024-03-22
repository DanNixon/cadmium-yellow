use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Line {
    /// Name of the line
    pub name: LineName,

    /// Identifiers of stations on this line, in the order they are visited by a train
    pub stations: Vec<String>,
}

/// A service line on the Tyne and Wear Metro.
#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LineName {
    /// The green line, between Airport and South Hylton.
    Green,

    /// The yellow line, between St James and South Shields.
    Yellow,
}

impl Display for LineName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Green => "Green",
                Self::Yellow => "Yellow",
            }
        )
    }
}
