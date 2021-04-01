mod socket;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
// use std::{fs, thread};

// async fn image(req: HttpRequest) -> HttpResponse {
//     let data = fs::read("./images/buffer0.jpg").unwrap();
//     return HttpResponse::Ok()
//         .content_type("image/jpeg")
//         .header("Content-Disposition", "inline")
//         .body(data);
// }

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    // thread::spawn(move || {
    //     // connection succeeded
    //     socket::start();
    // });
    // We'll bind to 127.0.0.1:8080
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
