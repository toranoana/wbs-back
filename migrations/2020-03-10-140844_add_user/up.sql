CREATE TABLE users
(
    -- 分散化する場合はエクステンション入れてuuid型で
    id           SERIAL PRIMARY KEY       NOT NULL,
    display_name VARCHAR(255)             NOT NULL,
    is_disabled  boolean                  NOT NULL default FALSE,
    created_at   timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
    updated_at   timestamp with time zone NOT NULL default CURRENT_TIMESTAMP
);
