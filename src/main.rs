use actix_web::{App, HttpRequest, HttpServer, Responder, web};

const PORT: &str = "8088";

async fn hello(_req: HttpRequest) -> impl Responder {
    "hello, world"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(hello))
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await
}
