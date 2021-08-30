CREATE TABLE holidays (
  id SERIAL PRIMARY KEY NOT NULL,
  holiday_name    VARCHAR(255)                 NOT NULL,
  target_at timestamp with time zone NOT NULL,
  created_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP,
  updated_at timestamp with time zone NOT NULL default CURRENT_TIMESTAMP
);