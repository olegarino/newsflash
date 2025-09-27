use actix_web::{App, Error, HttpResponse, HttpServer, web::{self, ThinData}, Responder};
use confik::{Configuration as _, EnvSource};
use deadpool_postgres::{Client, Pool};
use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;

mod config;
mod db;
mod errors;
mod models;

use self::{errors::PostgresError, models::User};

pub async fn get_users(ThinData(db_pool): web::ThinData<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(PostgresError::PoolError)?;

    let users = db::get_users(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_user(
    user: web::Json<User>,
    ThinData(db_pool): web::ThinData<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();

    let client: Client = db_pool.get().await.map_err(PostgresError::PoolError)?;

    let new_user = db::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    async fn home() -> impl Responder {
        HttpResponse::Ok().body("Welcome to the home page!")
    }

    let config = ExampleConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::ThinData(pool.clone()))
            .service(
            web::resource("/users")
                .route(web::post().to(add_user))
                .route(web::get().to(get_users)),
            )
            .route("/", web::get().to(home))
    })
        .bind(config.server_addr.clone())?
        .run();

    println!("Server running at http://{}/", config.server_addr);

    server.await
}