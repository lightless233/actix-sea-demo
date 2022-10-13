mod entities;
mod migrator;

use actix_web::middleware::{self, Logger};
use actix_web::{delete, get, post, put, web, App, HttpServer, Responder};
use entities::prelude::*;
use entities::user;
use migrator::Migrator;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, IntoActiveModel,
    ModelTrait, QueryFilter, Set,
};
use sea_orm_migration::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Debug, Deserialize, Serialize)]
struct APIResult<T> {
    code: u16,
    message: String,
    data: Option<T>,
}

#[derive(Debug, Deserialize, Serialize)]
struct APIResult2<'a, T> {
    code: u16,
    message: &'a str,
    data: Option<T>,
}

#[derive(Debug, Deserialize)]
struct AddUserParams {
    name: String,
    password: String,
}
#[put("/add")]
async fn add_user(state: web::Data<AppState>, body: web::Json<AddUserParams>) -> impl Responder {
    let model = user::ActiveModel {
        name: Set(body.name.to_owned()),
        password: Set(body.password.to_owned()),
        ..Default::default()
    };
    let row = model.save(&state.conn).await.unwrap();
    let mut result = HashMap::new();
    result.insert("id", row.id.unwrap());
    return web::Json(APIResult {
        code: 2000,
        message: "success".to_owned(),
        data: Some(result),
    });
}

#[derive(Debug, Deserialize)]
struct GetUserParams {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserInfo {
    id: i32,
    name: String,
    password: String,
}

#[get("/get")]
async fn get_user(state: web::Data<AppState>, params: web::Query<GetUserParams>) -> impl Responder {
    let name = &params.name;
    let row = User::find()
        .filter(user::Column::Name.eq(name.as_str()))
        .one(&state.conn)
        .await
        .unwrap();
    let api_result = if row.is_some() {
        let t = row.unwrap();
        let data = UserInfo {
            id: t.id,
            name: t.name,
            password: t.password,
        };
        APIResult {
            code: 2000,
            message: "success".to_owned(),
            data: Some(data),
        }
    } else {
        APIResult {
            code: 2000,
            message: "success".to_owned(),
            data: None,
        }
    };
    return web::Json(api_result);
}

#[derive(Debug, Deserialize)]
struct DeleteUserParams {
    id: i32,
}
#[delete("/delete")]
async fn delete_user(
    state: web::Data<AppState>,
    params: web::Json<DeleteUserParams>,
) -> impl Responder {
    let model = User::find_by_id(params.id).one(&state.conn).await.unwrap();
    let result = model.unwrap().delete(&state.conn).await.unwrap();

    let mut map = HashMap::new();
    map.insert("s", result.rows_affected);
    return web::Json(APIResult {
        code: 2000,
        message: "success".to_owned(),
        data: Some(map),
    });
}

#[post("/edit")]
async fn edit_user(state: web::Data<AppState>, params: web::Json<UserInfo>) -> impl Responder {
    let model = User::find_by_id(params.id).one(&state.conn).await.unwrap();
    let mut model = model.unwrap().into_active_model();
    model.name = Set(params.name.to_owned());
    model.password = Set(params.password.to_owned());
    model.update(&state.conn).await.unwrap();

    return web::Json(APIResult2 {
        code: 2000,
        message: "success",
        data: Some(params.id),
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("./log4rs.yml", Default::default()).expect("初始化日志系统失败！");

    let conn = Database::connect("sqlite://test.db?mode=rwc")
        .await
        .unwrap();

    Migrator::status(&conn).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let app_state = AppState { conn };
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .service(
                web::scope("/users")
                    .service(add_user)
                    .service(get_user)
                    .service(delete_user)
                    .service(edit_user),
            )
            .app_data(web::Data::new(app_state.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
