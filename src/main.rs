mod socket;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
// use std::{fs, thread};

// async fn image(req: HttpRequest) -> HttpResponse {
//     let data = fs::read("./images/buffer0.jpg").unwrap();
//     return HttpResponse::Ok()
//         .content_type("image/jpeg")
//         .header("Content-Disposition", "inline")
//         .body(data);
// }

async fn route_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match(req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from("Hello World"))),
        
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 8080).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(route_handler)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}