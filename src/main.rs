mod socket;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::body::Bytes;
use std::{fs, thread};
use tokio::time::{sleep, Duration};

// async fn image(req: HttpRequest) -> HttpResponse {
//     let data = fs::read("./images/buffer0.jpg").unwrap();
//     return HttpResponse::Ok()
//         .content_type("image/jpeg")
//         .header("Content-Disposition", "inline")
//         .body(data);
// }

async fn handle_video(_req: Request<Body>, video_path: String) -> Response<Body> {
    // let image = fs::read("./images/image.jpg").unwrap();

    let (sender, body) = Body::channel();

    // let mut resp = Response::new(Body::empty());

    tokio::spawn(async move {
        let mut sender = sender;
    
        loop {
            let path = format!("./images/{}", video_path);
            let image = fs::read(path).unwrap();

            let content_length = format!("Content-Length {}\n\n", image.len());

            sender.send_data(Bytes::from("--frame\n")).await.unwrap();
            sender.send_data(Bytes::from("Content-Type: image/jpeg\n")).await.unwrap();
            sender.send_data(Bytes::from(content_length)).await.unwrap();
            sender.send_data(Bytes::from(image)).await.unwrap();
            // sender.send_data(Bytes::from("\n\n")).await.unwrap();

            sleep(Duration::from_millis(100)).await; //delay
        }
    });
    
    
    let resp = Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "multipart/x-mixed-replace; boundary=--frame")
    .body(body);

    
    return resp.unwrap();
}

async fn route_handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    return match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from("GOOD"))),
        (&Method::GET, "/videos/carrack") => Ok(handle_video(req, String::from("carrack.jpg")).await),
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 8080).into();

    thread::spawn(move || {
        // connection succeeded
        socket::start();
    });

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(route_handler)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
