use diesel::{table, joinable, allow_tables_to_appear_in_same_query};

table! {
    runs (id) {
        id -> Int4,
        filename -> Varchar,
        experiment_number -> Int4,
        title -> Varchar,
        users -> Varchar,
        run_start -> Timestamp,
        run_end -> Timestamp,
        good_frames -> Int4,
        raw_frames -> Int4,
        additional_values -> Jsonb,
    }
}

table! {
    scripts (id) {
        id -> Int4,
        script -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::types::StateMapping;

    reductions (id) {
        id -> Int4,
        reduction_start -> Timestamp,
        reduction_end -> Timestamp,
        reduction_state -> StateMapping,
        script -> Int4, // This is a foreign key reference to the `scripts` table
        reduction_outputs -> Varchar,
    }
}

table! {
    runs_reductions (run, reduction) { // Use a tuple to specify a composite primary key
       run->Int4,
       reduction->Int4,
    }
}

joinable!(reductions->scripts(script));
joinable!(runs_reductions->reductions(reduction));
joinable!(runs_reductions->runs(run));

allow_tables_to_appear_in_same_query!(
    reductions,
    runs,
    runs_reductions,
    scripts,
);
