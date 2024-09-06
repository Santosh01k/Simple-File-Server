use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::path::PathBuf;

async fn file_server(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path: PathBuf = req.uri().path().trim_start_matches('/').into();
    
    if let Ok(mut file) = File::open(path).await {
        let mut contents = vec![];
        file.read_to_end(&mut contents).await.unwrap();
        Ok(Response::new(Body::from(contents)))
    } else {
        Ok(Response::builder()
            .status(404)
            .body(Body::from("File not found"))
            .unwrap())
    }
}

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(file_server)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Serving on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
