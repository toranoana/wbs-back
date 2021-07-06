-- Your SQL goes here
CREATE VIEW
    project_with_task_duration_view AS
    SELECT
           id,
           title,
           color,
           is_archived,
           started_at,
           ended_at,
           (select MIN(started_at) from tasks where project_id = projects.id) as min_started_at,
           (select MAX(ended_at) from tasks where project_id = projects.id) as max_ended_at
    FROM projects;
