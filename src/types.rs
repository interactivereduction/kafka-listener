use serde::{Serialize, Deserialize};
use serde_json::{Value};
use chrono::NaiveDateTime;
use diesel::SqlType;
use diesel::prelude::{Insertable};
use crate::schema::{runs, reductions};

#[derive(Debug, diesel_derive_enum::DbEnum, Serialize, Deserialize)]
pub enum State {
    Successful,
    Unsuccessful,
    Error,
    NotStarted,
}

#[derive(Serialize, Deserialize)]
pub struct KafkaNewRun {
    pub run_number: String,
    pub instrument: String,
    pub experiment_title: String,
    pub experiment_number: String,
    pub users: String,
    pub filepath: String,
    pub will_reduce: bool,
    #[serde(with = "iso_datetime")]
    pub run_start: NaiveDateTime,
    #[serde(with = "iso_datetime")]
    pub run_end: NaiveDateTime,
    pub good_frames: i32,
    pub raw_frames: i32,
    pub additional_values: Value,
}

mod iso_datetime {
    use chrono::{NaiveDateTime, DateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

    pub fn serialize<S>(
        date: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Insertable)]
#[table_name = "runs"]
pub(crate) struct SQLNewRun {
        pub(crate) filename: String,
        pub(crate) experiment_number: i32,
        pub(crate) title: String,
        pub(crate) users: String,
        pub(crate) run_start: NaiveDateTime,
        pub(crate) run_end: NaiveDateTime,
        pub(crate) good_frames: i32,
        pub(crate) raw_frames: i32,
        pub(crate) additional_values: Value,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct KafkaCompletedRun {
    status: State,
    #[serde(rename="status message")]
    status_message: Option<String>,
    #[serde(rename="run output")]
    run_output: Option<String>
}

#[derive(Insertable)]
#[table_name = "reductions"]
pub(crate) struct NewReduction {
    reduction_start: NaiveDateTime,
    reduction_end: NaiveDateTime,
    reduction_state: State,
    script: i32,
    reduction_outputs: String,
}