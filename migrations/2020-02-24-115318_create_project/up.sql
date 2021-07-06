--- Your SQL goes here
CREATE TABLE projects (
  -- 分散化する場合はエクステンション入れてuuid型で
  id SERIAL PRIMARY KEY NOT NULL,
  title VARCHAR(255) NOT NULL,
  color VARCHAR(7) NOT NULL,
  is_archived boolean NOT NULL default FALSE,
  started_at timestamp with time zone NOT NULL,
  ended_at timestamp with time zone NOT NULL,
  created_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
  updated_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP
);
create index projects_archived_and_start_index on projects (is_archived, started_at);

