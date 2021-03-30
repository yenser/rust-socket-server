mod socket;

use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use std::{fs, thread};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn image(req: HttpRequest) -> HttpResponse {

    let data = fs::read("./images/buffer0.jpg").unwrap();
    return HttpResponse::Ok()
        .content_type("image/jpeg")
        .header("Content-Disposition", "inline")
        .body(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    thread::spawn(move || {
        // connection succeeded
        socket::start();
      });

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(image))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}