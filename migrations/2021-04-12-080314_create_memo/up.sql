-- Your SQL goes here
CREATE TABLE memos (
  -- 分散化する場合はエクステンション入れてuuid型で
  id SERIAL PRIMARY KEY NOT NULL,
  task_id   int references tasks (id) NOT NULL,
  user_id      int references users (id)    NOT NULL,
  content VARCHAR(255) NOT NULL,
  created_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
  updated_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP
);