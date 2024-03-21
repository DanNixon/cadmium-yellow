use chrono::{DateTime, FixedOffset, TimeDelta};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct Train {
    /// Identifier of (I assume) the Metrocar that serves this service
    #[serde(rename = "trn")]
    id: String,

    /// Line that the train operates on
    line: crate::Line,

    /// The last reported position/activity of the train
    #[serde(flatten)]
    last_event: TrainEvent,

    /// When the train is due to arrive at the platform
    #[serde(rename = "dueIn")]
    due: TrainArrival,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TrainArrival {
    /// The train has arrived at the platform
    Arrived,

    /// The train will be immanently arriving at the platform
    Due,

    /// The train is due to arrive at the platform in a given amount of time
    DueIn(TimeDelta),
}

impl<'de> Deserialize<'de> for TrainArrival {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let minutes: i64 = Deserialize::deserialize(deserializer)?;
        if minutes == -1 {
            Ok(Self::Arrived)
        } else if minutes == 0 {
            Ok(Self::Due)
        } else {
            Ok(Self::DueIn(
                TimeDelta::try_minutes(minutes)
                    .expect("time in minutes should fit into a TimeDelta"),
            ))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
pub struct TrainEvent {
    /// Type of event/what has happened
    #[serde(rename = "lastEvent")]
    kind: TrainEventKind,

    /// Time that the event took place
    #[serde(rename = "lastEventTime")]
    time: DateTime<FixedOffset>,

    //Human friendly location that the event took place
    #[serde(rename = "lastEventLocation")]
    location: String,
}

#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrainEventKind {
    /// The train is approaching the platform indicated in the event.
    Approaching,

    /// The train has arrived at the platform indicated in the event.
    Arrived,

    /// (I assume) the train is ready to depart from the platform it is currently at.
    ReadyToStart,

    /// The train has departed from the platform indicated in the event.
    Departed,
}
