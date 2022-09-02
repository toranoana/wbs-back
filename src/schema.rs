// @generated automatically by Diesel CLI.

diesel::table! {
    holidays (id) {
        id -> Int4,
        holiday_name -> Varchar,
        target_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    memos (id) {
        id -> Int4,
        task_id -> Int4,
        user_id -> Int4,
        content -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    milestones (id) {
        id -> Int4,
        project_id -> Int4,
        confirmed_at -> Timestamptz,
        description -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        title -> Varchar,
        color -> Varchar,
        is_archived -> Bool,
        started_at -> Timestamptz,
        ended_at -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        project_id -> Int4,
        user_id -> Int4,
        task_name -> Varchar,
        started_at -> Timestamptz,
        ended_at -> Timestamptz,
        progress -> Int2,
        order_number -> Float8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        display_name -> Varchar,
        is_disabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(memos -> tasks (task_id));
diesel::joinable!(memos -> users (user_id));
diesel::joinable!(milestones -> projects (project_id));
diesel::joinable!(tasks -> projects (project_id));
diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    holidays,
    memos,
    milestones,
    projects,
    tasks,
    users,
);
