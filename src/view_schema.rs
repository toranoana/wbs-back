table! {
    project_with_task_duration_view (id) {
        id -> Int4,
        title -> Varchar,
        color -> Varchar,
        is_archived -> Bool,
        started_at -> Timestamptz,
        ended_at -> Timestamptz,
        min_started_at -> Nullable<Timestamptz>,
        max_ended_at -> Nullable<Timestamptz>,
    }
}
