use serde_json::from_slice;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use diesel::pg::PgConnection;
use r2d2::Pool;
use crate::schema;
use crate::utils::convert_kafka_to_sql_run;
use crate::types::{KafkaNewRun, KafkaCompletedRun, SQLNewRun};


pub(crate) async fn detected_runs(message: &[u8], db_pool: Pool<ConnectionManager<PgConnection>>) -> crate::Result<()> {
    let kafka_run: KafkaNewRun = from_slice(message)?;

    // If a reduction shouldn't happen, do not add it to the database, it is not a requested run.
    if kafka_run.will_reduce == false {
        return Ok(())
    }

    let db_run: SQLNewRun = convert_kafka_to_sql_run(kafka_run);
    let filename: String = db_run.filename.clone();

    let mut db_connection = db_pool.get().unwrap();

    diesel::insert_into(schema::runs::table)
        .values(db_run)
        .execute(&mut db_connection)
        .expect(format!("Unable to insert {}", filename).as_str());

    Ok(())
}

pub(crate) async fn completed_runs(message: &[u8], db_pool: Pool<ConnectionManager<PgConnection>>) -> crate::Result<()> {
    let run: KafkaCompletedRun = from_slice(message)?;

    Ok(())
}