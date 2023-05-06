use bytes::{Bytes, BytesMut};
use http::{Request, Response, header::ACCEPT_ENCODING};
use http_body::Body as _; // for Body::data
use hyper::Body;
use std::convert::Infallible;
use tokio::fs::{self, File};
use tokio_util::io::ReaderStream;
use tower::{Service, ServiceExt, ServiceBuilder, service_fn};
use tower_http::{compression::CompressionLayer, BoxError};

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Open the file.
    let file = File::open("Cargo.toml").await.expect("file missing");
    // Convert the file into a `Stream`.
    let stream = ReaderStream::new(file);
    // Convert the `Stream` into a `Body`.
    let body = Body::wrap_stream(stream);
    // Create response.
    Ok(Response::new(body))
}

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    let mut service = ServiceBuilder::new()
        // Compress responses based on the `Accept-Encoding` header.
        .layer(CompressionLayer::new())
        .service_fn(handle);

    // Call the service.
    let request = Request::builder()
        .header(ACCEPT_ENCODING, "gzip")
        .body(Body::empty())?;

    let response = service
        .ready()
        .await?
        .call(request)
        .await?;

    assert_eq!(response.headers()["content-encoding"], "gzip");

    // Read the body
    let mut body = response.into_body();
    let mut bytes = BytesMut::new();
    while let Some(chunk) = body.data().await {
        let chunk = chunk?;
        bytes.extend_from_slice(&chunk[..]);
    }
    let bytes: Bytes = bytes.freeze();

    // The compressed body should be smaller 🤞
    let uncompressed_len = fs::read_to_string("Cargo.toml").await?.len();
    assert!(bytes.len() < uncompressed_len);

    Ok(())
}

