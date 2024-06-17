#[macro_use]
extern crate diesel;

use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{App, delete, error, Error, get, HttpRequest, HttpResponse, HttpServer, middleware, post, Responder, web};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use diesel::{prelude::*, r2d2};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[get("/api/item/{item_id}")]
async fn get_item(
    pool: web::Data<DbPool>,
    item_id: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let item_uid = item_id.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let item = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::get_item(&mut conn, item_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match item {
        // item was found; return 200 response with JSON formatted item object
        Some(item) => HttpResponse::Ok().json(item),

        // item was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No item found with UID: {item_uid}")),
    })
}

#[post("/api/item")]
async fn new_item(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let item = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::new_item(&mut conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // item was added successfully; return 201 response with new item info
    Ok(HttpResponse::Created().json(item))
}

#[post("/api/item/{id}")]
async fn update_item(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    item_in: web::Json<models::Item>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let item_out = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::update_item(&mut conn, &id, &item_in)
    })
        .await?
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError)?;

    // item was added successfully; return 201 response with new item info
    Ok(HttpResponse::Accepted().json(item_out))
}

#[delete("/api/item/{id}")]
async fn delete_item(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let item_out = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::delete_item(&mut conn, &id)
    })
        .await?
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError)?;

    // item was added successfully; return 201 response with new item info
    Ok(HttpResponse::Ok().json(item_out))
}

#[get("/{filename:.*}")]
async fn index(req: HttpRequest) -> Result<NamedFile, Error> {
    let query_path = req.match_info().query("filename");
    let mut path = PathBuf::from(format!("static/{query_path}"));
    if !path.is_file() || !path.exists()  {
        path = PathBuf::from(format!("static/{query_path}/index.html"));
    }
    tracing::error!("Trying to read: [{path:#?}]");
    let file = NamedFile::open(path)?;
    Ok(file)
}

fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
        InitError = (),
    >,
> {
    let pool = initialize_db_pool();
    App::new()
        // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
        .app_data(web::Data::new(pool))
        // add request logger middleware
        .wrap(middleware::Logger::default())
        // add route handlers
        .service(get_item)
        .service(new_item)
        .service(update_item)
        .service(delete_item)
        // This must be last, as the default
        .service(index)
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        create_app()
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

/// Initialize database connection pool, on file: ./.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    pool.get().unwrap().run_pending_migrations(MIGRATIONS).unwrap();
    pool
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use super::*;

    #[actix_web::test]
    async fn api_item_crud() {
        dotenvy::dotenv().ok();

        let app = test::init_service(
            create_app(),
        ).await;

        // Create
        let req = test::TestRequest::post().uri("/api/item").to_request();
        let res1: models::Item = test::call_and_read_body_json(&app, req).await;

        // Update with name:
        let res_new = models::Item {
            id: res1.id,
            description: "This is lasagna".to_string()
        };
        let uri = format!("/api/item/{}", &res1.id);
        let req = test::TestRequest::post().uri(uri.as_str()).set_json(&res_new).to_request();
        let res2: models::Item = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res_new, res2);

        // Get:
        let uri = format!("/api/item/{}", &res2.id);
        let req = test::TestRequest::get().uri(uri.as_str()).to_request();
        let res3: models::Item = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res_new, res3);

        // Delete:
        let uri = format!("/api/item/{}", &res1.id);
        let req = test::TestRequest::delete().uri(uri.as_str()).to_request();
        let res4: models::Item = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res_new, res4);

        // Get:
        let uri = format!("/api/item/{}", &res1.id);
        let req = test::TestRequest::get().uri(uri.as_str()).to_request();
        let res5 = test::call_service(&app, req).await;
        assert_eq!(res5.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn static_get() {
        dotenvy::dotenv().ok();

        let app = test::init_service(
            create_app(),
        ).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(StatusCode::OK, res.status());
    }
}
