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
pub(crate) struct KafkaNewRun {
    pub(crate) run_number: i32,
    pub(crate) instrument: String,
    pub(crate) experiment_title: String,
    pub(crate) experiment_number: String,
    pub(crate) users: String,
    pub(crate) filepath: String,
    pub(crate) will_reduce: bool,
    pub(crate) run_start: NaiveDateTime,
    pub(crate) run_end: NaiveDateTime,
    pub(crate) good_frames: i32,
    pub(crate) raw_frames: i32,
    pub(crate) additional_values: Value,
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