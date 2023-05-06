#![cfg_attr(debug_assertions, allow(unused_imports))]

use axum_macros::debug_handler;

// command line arguments
use clap::Parser;

// files
use std::fs;

use std::collections::HashMap;
// time
use std::time::Duration;

// axum web framework
use std::net::SocketAddr;
use axum::{
    body::StreamBody, // return files dynamically
    extract::{Query, Path},
    headers::{Header},
    routing::{get, post},
    http::{Request,StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
    response::{AppendHeaders,Html,IntoResponse},
    Json, Router,
};
use tokio_util::io::ReaderStream;

use tower_http::{compression::CompressionLayer, BoxError};

// serialization
use serde_derive::{Deserialize, Serialize};

// read from toml
use toml;

use poke_rs_api::pkmn::Pokemon;

// Command line arguments
#[derive(Parser, Debug)]
struct Args {

}

static CONTENT_TYPE_TEXT_TOML: &'static str = "text/toml"; 
static CONTENT_TYPE_AUDIO_OGG: &'static str = "audio/ogg";

#[tokio::main]
async fn main() {
    let mut port : u16 = 3001;
    let mut requests_before_limit : u64 = 5;
    let mut max_requests_before_backpressure : usize = 100;
    // collect command line arguments
    //let args: Vec<String> = env::args().collect();
    let cmd_arg_matches = clap::Command::new("poke-rs-api")
        .arg(
            clap::Arg::new("port")
            .short('p')
            .long("port")
            .action(clap::ArgAction::Set)
            .value_name("PORT")
            .help("Set port for server to listen on")
        )
        .get_matches();
    
    if let Some(p) = cmd_arg_matches.get_one::<String>("port") {
        
        match p.parse::<u16>() {
            Ok(num) => {port = num;},
            Err(e) => (),
        }
        println!("Value for port is now: {}", port);
    }
    
    // Logging
    //tracing_subscriber::fmt::init()
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    
    // let method_router : axum::routing::method_routing::MethodRouter = get(|Path(id):Path<u32>| async {});
    // build web application with routes
    let get_id_route : axum::routing::method_routing::MethodRouter  = get(get_id);
    let user_route = Router::new().route("/:id", get_id_route.clone());
    let team_route = Router::new().route("/", post(|| async {}));
    
    let api_routes = Router::new()
        .nest("/users", user_route)
        .nest("/teams", team_route);

    let app_with_nested_routes = Router::new()
        .nest("/api", api_routes);
    // App now accepts
    // - GET /api/users/:id
    // - POSpokT /api/teams
    // 
    // nested api
    // /api
    let berry_route: axum::Router =Router::new().route("/berry/:id", get(return_json_file_berry));
    let move_route=Router::new().route("/move/:id", get(placeholder));
    let pkmn_route=Router::new().route("/pokemon/:id", get(placeholder));
    let pokeapiv2_route = Router::new()
        .nest("/pokeapi/v2/", berry_route.clone())
        .nest("/pokeapi/v2/", move_route.clone())
        .nest("/pokeapi/v2/", pkmn_route.clone())
    ;

    let app = Router::new()
        // `GET -> /`
        .route("/:id", get_id_route.clone())
        .route("/", get(get_root))
        .route("/", post(post_default))
        .route("/id", get(get_read))
        .route("/quote", get(return_quote))
        .route("/todos", get(get_todos))
        .route("/toml", get(return_toml_handler))
        .route("/nanoid", get(poke_rs_api::util::get_nanoid))
        .route("/pokeapi", get(placeholder))
        .nest("/pokeapi/v2", berry_route)
        .nest("/pokeapi/v2", move_route)
        .nest("/pokeapi/v2", pkmn_route)
        // timeout at 30 seconds
        .layer(
            
            tower::ServiceBuilder::new()
                .layer(axum::error_handling::HandleErrorLayer::new(handle_timeout_error))
                // `cargo add tower --features timeout`
                .timeout(std::time::Duration::from_secs(30))
        )
    ;
    
    let domain_all = "0.0.0.0";
    
    // https://crates.io/crates/axum
    // run app
    let localhost = ([127,0,0,1],port);
    let socket_all = ([0,0,0,0], port);
    
    let addr = SocketAddr::from(localhost);
    
    tracing::debug!("listening on {}", addr);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    //let listener = tokio::net::TcpListener::bind(format!("{}:{}",domain_all,port));
    //axum::serve(listener, app).await.unwrap();
}
// end-main

async fn return_json_file_berry(
    axum::extract::Path(id): axum::extract::Path<u32>)-> Result<impl IntoResponse,StatusCode>{
    
    let file_name = format!("{}.json",id);
    let file_name_path = format!("../berry/{}.json",id);

    let file_ok = match tokio::fs::File::open(&file_name_path).await {
        Ok(file) => {
            println!("Getting {}",&file_name);
            file},
        Err(err) => return Err(StatusCode::NOT_FOUND/*, format!("File not found: {}", err)*/),
    };
    // Convert `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file_ok);
    // Convert the `Stream` into an `axum::body::HttpBody`
    let http_body = StreamBody::new(stream);

    let a = format!("attachment; filename=\"{}\"", file_name.clone());
    let a_str: &str = &a[..];
    let headers = //[(HeaderName, &'static str) ; 2]
        [
            (header::CONTENT_TYPE, "text/json; charset=utf-8".to_string()),
            (header::CONTENT_DISPOSITION, a),
        ]
    ;

    Ok((headers,http_body))
}

// Handle timeout
async fn handle_timeout_error(
    method: axum::http::Method,
    uri: axum::http::Uri,
    err: axum::BoxError
) -> (axum::http::StatusCode, String){
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        format!("`{} {}` failed with {}", method, uri, err),
    )
}

#[debug_handler]
async fn placeholder() -> &'static str{
    "you got placeholder"
}


// index
async fn get_root() -> &'static str {
    "You got root"
}

// read
async fn get_id(Path(id): Path<u32>) -> String {
    "endpoint: read placedholder";
    id.to_string()
}

// create
async fn post_default() -> &'static str {
    "endpoint: create placeholder"
}

// get read
async fn get_read() -> &'static str {
    "endpoint: get_read"
}

// update
async fn put_id() -> &'static str {
    "endpoint: update placeholder"
}

// destroy
async fn delete_id() -> &'static str {
    "endpoint: delete placedholder"
}

async fn get_todos() -> &'static str {
    "endpoint: todos placeholder"
}

// extract data from url, i.e. extract query

#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

// This will parse query strings like `?page=2&per_page=30` into `Pagination`
// structs.
async fn list_things(pagination: Query<Pagination>) {
    let pagination: Pagination = pagination.0;

    // ...
}

// usage: let app = Router::new().route("/list_things", get(list_things));
// a bad query will -> 400 Bad Request

// Return files dynamically: https://github.com/tokio-rs/axum/discussions/608
async fn return_toml_handler() -> Result<impl IntoResponse,StatusCode> {
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

async fn return_quote() -> &'static str {
    r#""Give a man a fire and he's warm for a day, 
but set fire to him and he's warm for the rest of his life."
    - Terry Pratchett, Jingo"#
}