-- Your SQL goes here
-- プロジェクトIDで必ず絞られてorder_numberで順番が決まるのでindex作成
-- ただし、project_idは初回に入れられた後更新されないので単体では作成しない

create index tasks_project_order_number_index on tasks (project_id, order_number);
create index tasks_user_foreign_key_index on tasks (user_id);
