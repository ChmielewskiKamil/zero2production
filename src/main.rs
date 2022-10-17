use actix_web::{web, App, HttpRequest, HttpServer, Responder};

/**
 * This is the handler
 * App is trying to match the HTTP verb
 * to a matching registered endpoint -->
 * handler is used
 */
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // this request is only passed to the handler
            // if the method is GET
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
