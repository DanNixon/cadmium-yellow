use serde::Deserialize;

/// A service line on the Tyne and Wear Metro.
#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Line {
    /// The green line, between Airport and South Hylton.
    Green,

    /// The yellow line, between St James and South Shields.
    Yellow,
}
