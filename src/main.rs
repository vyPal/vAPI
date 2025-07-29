use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder, Result};
use env_logger::Env;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

#[utoipa::path(
    responses(
        (status = 200, description = "Pings the server"),
    ),
)]
#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    responses(
        (status = 200, description = "Find out if a number is even", body = String),
    ),
    params(
        ("number", description = "The number to compare")
    )
)]
#[get("/is_even/{number}")]
async fn is_even(path: web::Path<i64>) -> Result<String> {
    Ok((path.into_inner() % 2 == 0).to_string())
}

#[utoipa::path(
    responses(
        (status = 200, description = "Find out if a number is odd", body = String),
    ),
    params(
        ("number", description = "The number to compare")
    )
)]
#[get("/is_odd/{number}")]
async fn is_odd(path: web::Path<i64>) -> Result<String> {
    Ok((path.into_inner() % 2 != 0).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
    )]
    struct ApiDoc;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            .service(ping)
            .service(is_even)
            .service(is_odd)
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
