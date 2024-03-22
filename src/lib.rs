mod client;
mod error;
mod types;

pub use self::{
    client::{Client, DataSource},
    error::{Error, Result},
    types::{Line, LineName, Platform, Station, Train, TrainArrival, TrainEvent, TrainEventKind},
};
