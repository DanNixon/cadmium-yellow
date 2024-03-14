use chrono::{DateTime, FixedOffset, TimeDelta};
use serde::Deserialize;

fn deserialize_due_time_to_optional_timedelta<'de, D>(
    deserializer: D,
) -> Result<Option<TimeDelta>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let minutes: i64 = Deserialize::deserialize(deserializer)?;
    if minutes == -1 {
        Ok(None)
    } else {
        Ok(Some(
            TimeDelta::try_minutes(minutes).expect("time in minutes should fit into a TimeDelta"),
        ))
    }
}

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

    /// Estimated time until the train arrives on the platform, None if the train has arrived at the platform.
    #[serde(
        rename = "dueIn",
        deserialize_with = "deserialize_due_time_to_optional_timedelta"
    )]
    due_in: Option<TimeDelta>,
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
