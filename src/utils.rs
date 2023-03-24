use crate::types::{KafkaNewRun, SQLNewRun};

pub(crate) fn convert_kafka_to_sql_run(kafka_run: KafkaNewRun) -> SQLNewRun {
    return SQLNewRun {
        filename: kafka_run.filepath,
        experiment_number: kafka_run.experiment_number.parse().unwrap(),
        title: kafka_run.experiment_title,
        users: kafka_run.users,
        run_start: kafka_run.run_start,
        run_end: kafka_run.run_end,
        good_frames: kafka_run.good_frames,
        raw_frames: kafka_run.raw_frames,
        additional_values: kafka_run.additional_values,
    }
}