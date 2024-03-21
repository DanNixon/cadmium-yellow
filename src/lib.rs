mod client;
mod error;
mod types;

pub use self::{
    client::{Client, DataSource},
    error::{Error, Result},
    types::{Line, Platform, Station, Train, TrainEvent, TrainEventKind},
};
