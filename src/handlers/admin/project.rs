use crate::{db::manager::DataPgPool, graphql::schema::project::UpdateProject};
use actix_web::{Error, HttpResponse, Result, error, http::header, web};
use crate::db::repositories::project_repository::ProjectRepository;
use crate::handlers::api::v1::project::parameters::{ProjectPath};
use serde::{Serialize, Deserialize};

pub async fn index(tmpl: web::Data<tera::Tera>, pool: DataPgPool) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let projects = ProjectRepository::search_pool_projects(&pool, false);
    match projects {
      Ok(t) => {
        ctx.insert("projects", &t);
        Ok(HttpResponse::Ok().content_type("text/html").body(tmpl.render("project/index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?))
      },
      Err(e) => {
        Err(error::ErrorInternalServerError(e))
      }
    }
}

pub async fn delete(params: web::Form<FormParams>, pool: DataPgPool) -> Result<HttpResponse, Error> {
  let update_project = UpdateProject{ title: None , color: None, is_archived: Some(true), started_at: None, ended_at: None };
  ProjectRepository::update_pool_project(params.id, update_project, &pool);

  Ok(HttpResponse::Found()
      .header(header::LOCATION, "/admin/project/")
      .finish()
  )
}

pub async fn update(params: web::Form<UpdateFormParams>, project_path: web::Path<ProjectPath>, pool: DataPgPool) -> Result<HttpResponse, Error> {
  let update_project = UpdateProject{ title: Some(params.title.clone()) , color: Some(params.color.clone()), is_archived: None, started_at: None, ended_at: None };
  ProjectRepository::update_pool_project(project_path.project_id, update_project, &pool);

  Ok(HttpResponse::Found()
      .header(header::LOCATION, "/admin/project/")
      .finish()
  )
}
pub async fn edit(tmpl: web::Data<tera::Tera>,
  project_path: web::Path<ProjectPath>, pool: DataPgPool) -> Result<HttpResponse, Error> {
  let mut ctx = tera::Context::new();
  let project = ProjectRepository::find_pool_project(&pool,project_path.project_id);

  match project {
    Ok(t)=>{
      ctx.insert("project", &t);
      Ok(HttpResponse::Ok().content_type("text/html").body(tmpl.render("project/edit.html", &ctx)
      .map_err(|_| error::ErrorInternalServerError("Template error"))?))
    },
    Err(e)=>{
      Err(error::ErrorInternalServerError(e))
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct FormParams {
    id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateFormParams {
  id: i32,
  title: String,
  color: String,
}
