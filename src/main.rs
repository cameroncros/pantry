#[macro_use]
extern crate diesel;

use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{
    delete, error, get, middleware, post, put, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Responder,
};
use diesel::{prelude::*, r2d2};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::models::Item;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
#[utoipa::path(
    responses(
        (status = 200, description = "Got the item", body = Item),
    ),
    params(("id" = u64, Path, description = "Item id")),
)]
#[get("/api/item/{id}")]
async fn get_item(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let item_uid = id.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let item = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::get_item(&mut conn, item_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(item))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got the items", body = Item),
    )
)]
#[get("/api/all_items")]
async fn get_all_items(
    pool: web::Data<DbPool>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let item = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::get_all_items(&mut conn)
    })
        .await?
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(item))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Added new item", body = Item),
        (status = 500, description = "Failed to add new item"),
    ),
)]
#[post("/api/item")]
async fn new_item(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
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
#[utoipa::path(
    responses(
        (status = 201, description = "Updated item", body = Item),
        (status = 500, description = "Failed to update"),
    ),
    params(("id" = u64, Path, description = "Item id")),
    request_body(content = Item, description = "Item", content_type = "application/json"),
)]
#[put("/api/item/{id}")]
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
#[utoipa::path(
    responses(
        (status = 200, description = "Deleted Item", body = Item),
        (status = 500, description = "Failed to delete"),
    ),
    params(("id" = u64, Path, description = "Item id")),
)]
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

    // item was added successfully; return 200 response with new item info
    Ok(HttpResponse::Ok().json(item_out))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got file", body = String)
    ),
)]
#[get("/{filename:.*}")]
async fn index(req: HttpRequest) -> Result<NamedFile, Error> {
    let query_path = req.match_info().query("filename");
    let mut path = PathBuf::from(format!("static/{query_path}"));
    if !path.is_file() || !path.exists() {
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
    #[derive(OpenApi)]
    #[openapi(paths(get_item, new_item, update_item, delete_item, index),
        components(schemas(Item)))]
    struct ApiDoc;

    let pool = initialize_db_pool();
    App::new()
        // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
        .app_data(web::Data::new(pool))
        // add request logger middleware
        .wrap(middleware::Logger::default())
        // add route handlers
        .service(get_item)
        .service(get_all_items)
        .service(new_item)
        .service(update_item)
        .service(delete_item)
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", ApiDoc::openapi()))
        // This must be last, as the default
        .service(index)
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(create_app)
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

    pool.get()
        .unwrap()
        .run_pending_migrations(MIGRATIONS)
        .unwrap();
    pool
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use chrono::NaiveDate;

    use super::*;

    #[actix_web::test]
    async fn api_item_crud() {
        dotenvy::dotenv().ok();

        let app = test::init_service(create_app()).await;

        // Create
        let req = test::TestRequest::post().uri("/api/item").to_request();
        let res1: models::Item = test::call_and_read_body_json(&app, req).await;

        // Update with name:
        let res_new = models::Item {
            id: res1.id,
            description: "This is lasagna".to_string(),
            date: Some(NaiveDate::default())
        };
        let uri = format!("/api/item/{}", &res1.id);
        let req = test::TestRequest::put()
            .uri(uri.as_str())
            .set_json(&res_new)
            .to_request();
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
        assert_eq!(res5.status(), StatusCode::OK);

        // Get All:
        let uri = "/api/all_items";
        let req = test::TestRequest::get().uri(uri).to_request();
        let res5 = test::call_service(&app, req).await;
        assert_eq!(res5.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn static_get() {
        dotenvy::dotenv().ok();

        let app = test::init_service(create_app()).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(StatusCode::OK, res.status());
    }
}
