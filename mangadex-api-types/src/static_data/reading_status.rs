use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadingStatus {
    Completed,
    Dropped,
    OnHold,
    PlanToRead,
    Reading,
    ReReading,
}
