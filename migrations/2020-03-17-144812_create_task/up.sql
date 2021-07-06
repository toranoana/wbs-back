-- Your SQL goes here
CREATE TABLE tasks
(
    -- 分散化する場合はエクステンション入れてuuid型で
    id           SERIAL PRIMARY KEY           NOT NULL,
    project_id   int references projects (id) NOT NULL,
    user_id      int references users (id)    NOT NULL,
    task_name    VARCHAR(255)                 NOT NULL,
    started_at   timestamp with time zone     NOT NULL,
    ended_at     timestamp with time zone     NOT NULL,
    progress     smallint                     NOT NULL default 0,
    order_number int                          NOT NULL default 0,
    created_at   timestamp with time zone     NOT NULL default CURRENT_TIMESTAMP,
    updated_at   timestamp with time zone     NOT NULL default CURRENT_TIMESTAMP
)
