use crate::db::manager::DataPgPool;
use crate::db::models::task::TaskNewForm;
use crate::db::repositories::task_repository::TaskRepository;
use crate::handlers::api::v1::project::parameters::{
    ProjectPath, Task, TaskImportRequest, TaskImportResponse,
};
use actix_web::error::ErrorInternalServerError;
use actix_web::{web, HttpResponse, Result};
use chrono::{Local, NaiveDate};

///
/// Taskの配列を受け取って一括でインサートを行うハンドラ
/// TODO: エラーでJSON返せるように
/// TODO: ロジックをどこかに持っていく
///
pub async fn import(
    project_path: web::Path<ProjectPath>,
    import_request: web::Json<TaskImportRequest>,
    pool: DataPgPool,
) -> Result<HttpResponse> {
    // TODO: リポジトリかどっかに持っていきたい
    let mut tasks: Vec<TaskNewForm> = vec![];
    let mut invalid_lines: Vec<i32> = vec![];
    for (idx, t) in import_request.tasks.iter().enumerate() {
        let start = NaiveDate::parse_from_str(&t.started_at, "%Y-%m-%d");
        let end = NaiveDate::parse_from_str(&t.ended_at, "%Y-%m-%d");

        if start.is_ok() && end.is_ok() {
            tasks.push(TaskNewForm {
                project_id: project_path.project_id,
                user_id: t.user_id,
                task_name: &t.task_name,
                // if文でのチェックがあるのでここの値は安全
                started_at: start.unwrap().and_hms(00, 00, 00),
                ended_at: end.unwrap().and_hms(23, 59, 59),
                progress: t.progress.unwrap_or(0),
                order_number: t.order_number,
                created_at: Local::now().naive_local(),
                updated_at: Local::now().naive_local(),
            });
        } else {
            invalid_lines.push(idx as i32);
        }
    }

    let ret_tasks =
        TaskRepository::batch_insert_task(pool, &tasks).map_err(ErrorInternalServerError)?;
    let inserted_tasks: Vec<Task> = ret_tasks
        .iter()
        .map(|t| Task {
            user_id: t.user_id,
            task_name: t.task_name.clone(),
            started_at: t.started_at.format("%Y-%m-%d").to_string(),
            ended_at: t.ended_at.format("%Y-%m-%d").to_string(),
            progress: Some(t.progress),
            order_number: Some(t.order_number),
        })
        .collect();

    Ok(HttpResponse::Ok().json(TaskImportResponse {
        inserted_tasks,
        error_lines: invalid_lines,
    }))
}
