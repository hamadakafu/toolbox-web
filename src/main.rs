use std::env;
use std::path::Path;

use actix::Actor;
use actix_web::{dev::ServiceRequest, App, HttpServer};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use sqlx::migrate::Migrator;

mod handlers;
mod scheduler;
use handlers::text_handler::*;
use scheduler::Scheduler;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let port = env::var("PORT")?;
    let database_url = env::var("DATABASE_URL")?;
    let db_pool = sqlx::postgres::PgPool::connect(&database_url).await?;

    let m = Migrator::new(Path::new("./migrations")).await?;
    m.run(&db_pool).await?;

    Scheduler.start();

    HttpServer::new(move || {
        let basicauth = HttpAuthentication::basic(validator);
        App::new()
            .wrap(basicauth)
            // cloneが必要なのはHttpServer::newがclosureを2回呼び出すかも知れないので
            .data(db_pool.clone())
            .service(get_text)
            .service(create_text)
            .service(delete_text)
            .service(hello)
    })
    .bind("0.0.0.0:".to_owned() + &port)?
    .run()
    .await?;

    Ok(())
}

async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> actix_web::Result<ServiceRequest> {
    if credentials.user_id() == &env::var("BASIC_AUTH_USERNAME").unwrap()
        && credentials.password().unwrap() == &env::var("BASIC_AUTH_PASSWORD").unwrap()
    {
        Ok(req)
    } else {
        Err(actix_web::error::ErrorNetworkAuthenticationRequired(
            "Don't allow",
        ))
    }
}
