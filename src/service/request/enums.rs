use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub enum RequestMessage {}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DateBool {
    Date(NaiveDate),
    Bool(bool),
}
