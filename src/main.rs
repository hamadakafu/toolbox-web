use std::env;
use std::path::Path;

use actix::Actor;
use actix_web::{dev::ServiceRequest, web, App, HttpServer};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use sqlx::migrate::Migrator;
use tera::Tera;

mod handlers;
mod scheduler;
use handlers::{pbkdf2_handler, text_handler};
use scheduler::Scheduler;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let port = env::var("PORT")?;
    let database_url = env::var("DATABASE_URL")?;
    let db_pool = sqlx::postgres::PgPool::connect(&database_url).await?;

    {
        let m = Migrator::new(Path::new("./migrations")).await?;
        m.run(&db_pool).await?;
    }

    Scheduler.start();

    // db_poolのclone()が必要なのはHttpServer::newがclosureを2回呼び出すかも知れないので．
    // moveすると1つのclosureしか所有できない．
    HttpServer::new(move || {
        let basicauth = HttpAuthentication::basic(validator);
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        App::new()
            .service(
                web::scope("/texts")
                    .wrap(basicauth)
                    .app_data(web::Data::new(db_pool.clone()))
                    .route("/{id}", web::get().to(text_handler::get_text))
                    // ""と"/"は別物なので注意する
                    .route("", web::post().to(text_handler::create_text))
                    .route("/{id}", web::delete().to(text_handler::delete_text))
                    .route("/hello", web::get().to(text_handler::hello)),
            )
            .service(
                web::scope("/pbkdf2")
                    .app_data(web::Data::new(tera))
                    .route("/index.html", web::get().to(pbkdf2_handler::index)),
            )
            .route("/hello", web::get().to(text_handler::hello))
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
