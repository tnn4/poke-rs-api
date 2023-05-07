// api
use serde_derive::Deserialize;
use axum::{
    body::StreamBody, // return files dynamically
    extract::{Query, Path, State, Extension},
    headers::{Header},
    routing::{get, post},
    http::{Request,StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
    response::{AppendHeaders,Html,IntoResponse},
    Json, Router,
};
use tokio_util::io::ReaderStream;

// read
pub async fn get_id(Path(id): Path<u32>) -> String {
    "endpoint: read placedholder";
    id.to_string()
}

// extract data from url, i.e. extract query

#[derive(Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}

// This will parse query strings like `?page=2&per_page=30` into `Pagination`
// structs.
pub async fn list_things(pagination: Query<Pagination>) {
    let pagination: Pagination = pagination.0;

    // ...
}

// usage: let app = Router::new().route("/list_things", get(list_things));
// a bad query will -> 400 Bad Request

// Return files dynamically: https://github.com/tokio-rs/axum/discussions/608
pub async fn return_toml_handler() -> Result<impl IntoResponse,StatusCode> {
    // `File implements `AsyncRead`
    let file = match tokio::fs::File::open("Cargo.toml").await {
        Ok(file) => file,
        Err(err) => return Err(StatusCode::NOT_FOUND/*, format!("File not found: {}", err)*/),
    };
    // Convert `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // Convert the `Stream` into an `axum::body::HttpBody`
    let http_body = StreamBody::new(stream);

    let headers = //[(HeaderName, &'static str) ; 2]
        [
            (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"Cargo.toml\""),
        ]
    ;

    Ok((headers,http_body))
}