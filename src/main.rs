use actix_web::{delete, get, post, put, web, App, HttpServer, Responder};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::prelude::*;
use serde::Deserialize;
use migrator::Migrator;
use entities::prelude::*;

mod entities;
mod migrator;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[put("/add")]
async fn add_user() -> impl Responder {
    return "123";
}

#[derive(Debug, Deserialize)]
struct GetUserParams{
    name: String,
}

#[get("/get")]
async fn get_user(state: web::Data<AppState>, params: web::Query<GetUserParams>) -> impl Responder {
    let name = &params.name;
    let conn = &state.conn;
    let x = User::find

    return "123";
}

#[delete("/delete")]
async fn delete_user() -> impl Responder {
    return "123";
}

#[post("/edit")]
async fn edit_user() -> impl Responder {
    return "123";
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("./log4rs.yml", Default::default()).expect("初始化日志系统失败！");

    let conn = Database::connect("sqlite://test.db?mode=rwc")
        .await
        .unwrap();

    Migrator::status(&conn).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/users")
                    .service(add_user)
                    .service(get_user)
                    .service(delete_user)
                    .service(edit_user),
            )
            .app_data(web::Data::new(AppState { conn }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
