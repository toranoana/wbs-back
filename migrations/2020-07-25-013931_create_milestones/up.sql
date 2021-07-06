CREATE TABLE milestones
(
    -- 分散化する場合はエクステンション入れてuuid型で
    id           SERIAL PRIMARY KEY           NOT NULL,
    project_id   int references projects (id) NOT NULL,
    confirmed_at timestamp with time zone     NOT NULL, -- 確認日時
    description  VARCHAR(255)                 NOT NULL,
    created_at   timestamp with time zone     NOT NULL default CURRENT_TIMESTAMP,
    updated_at   timestamp with time zone     NOT NULL default CURRENT_TIMESTAMP
)