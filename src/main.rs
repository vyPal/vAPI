use actix_web::{get, middleware::Logger, web::{self, Redirect}, App, HttpResponse, HttpServer, Responder, Result};
use env_logger::Env;
use utoipa::{
    OpenApi,
};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use is_even_or_odd::IsEvenOrOdd;

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

#[utoipa::path(
    responses(
        (status = 200, description = "Find out if a number is even or odd", body = String),
    ),
    params(
        ("number", description = "The number to compare")
    )
)]
#[get("/is_even_or_odd/{number}")]
async fn is_even_or_odd_handler(path: web::Path<i64>) -> Result<String> {
    Ok(path.into_inner().is_even_or_odd().to_string())
}

#[get("/{_:.*}")]
async fn redir() -> impl Responder {
    Redirect::to("/swagger-ui/")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        info(description = "A random API ig. Code available on https://github.com/vyPal/vAPI")
    )]
    struct ApiDoc;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::new("%{X-Forwarded-For}i %r %s %b %{Referer}i %{User-Agent}i %T")))
            .service(ping)
            .service(is_even)
            .service(is_odd)
            .service(is_even_or_odd_handler)
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
            .service(redir)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
