use actix_web::{
    delete, get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Pool, Postgres, Row};

#[derive(sqlx::FromRow, Debug, Clone)]
struct Text {
    uuid: String,
    value: String,
    /// rfc3339 and ISO8601
    created_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTextReq {
    value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GetTextRes {
    value: String,
}

pub async fn hello() -> impl Responder {
    println!("hello");
    HttpResponse::Ok().body("hello")
}

pub async fn get_text(
    pool: Data<Pool<Postgres>>,
    path: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let id = &path.into_inner();
    let d = if id == "latest" {
        pool.fetch_one(sqlx::query!(
            "select value from texts order by created_at desc",
        ))
        .await
        .unwrap()
    } else {
        pool.fetch_one(sqlx::query!("select value from texts where uuid = $1", id))
            .await
            .unwrap()
    };
    let value: String = d.try_get(0).unwrap();
    Ok(HttpResponse::Ok().json(GetTextRes { value }))
}

pub async fn create_text(
    pool: Data<Pool<Postgres>>,
    json: web::Json<CreateTextReq>,
) -> impl Responder {
    let uuid = uuid::Uuid::new_v4().to_string();
    pool.execute(sqlx::query!(
        r#"insert into texts (uuid, value, created_at) values ($1 , $2, $3)"#,
        uuid,
        &json.value,
        chrono::Utc::now().to_rfc3339(),
    ))
    .await
    .unwrap();
    HttpResponse::Ok().body("ok")
}

pub async fn delete_text(pool: Data<Pool<Postgres>>, path: web::Path<String>) -> impl Responder {
    let id = &path.into_inner();
    if id == "old" {
        let yesterday = chrono::Utc::now() - Duration::days(1);
        pool.execute(sqlx::query!(
            r#"delete from texts where created_at < $1"#,
            yesterday.to_rfc3339()
        ))
        .await
        .unwrap();
    } else {
        pool.execute(sqlx::query!(r#"delete from texts where uuid = $1"#, id))
            .await
            .unwrap();
    }
    HttpResponse::Ok().body("ok")
}
