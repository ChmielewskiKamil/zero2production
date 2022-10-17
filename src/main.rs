use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

/**
 * This is the handler
 * App is trying to match the HTTP verb
 * to a matching registered endpoint -->
 * handler is used
 */

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        // this request is only passed to the handler
        // if the method is GET
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
