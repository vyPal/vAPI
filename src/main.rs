use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use env_logger::Env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
