// poke_rs_api
#![cfg_attr(debug_assertions, allow(unused_imports))]

// use std::path::Path;

use std::io::{Read, BufReader,};
use std::fs::File;

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
    extract::{Query, Path, State, Extension},
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
use toml::Table;
use poke_rs_api::pkmn::Pokemon;
use std::sync::Arc;

// Stores state to pass between handlers, handlers can't take direct arguments without using them as closures
#[derive(Clone,Serialize, Deserialize)]
struct AppState {
    berry_m: toml::Table,
    //move_m: toml::Table,
    pokemon_m: toml::Table,
    //m: HashMap<String,String>,
}

static CONTENT_TYPE_TEXT_TOML: &'static str = "text/toml"; 
static CONTENT_TYPE_AUDIO_OGG: &'static str = "audio/ogg";


// const POKEMAP: toml::Table = init_pokemap();

// main
#[tokio::main]
async fn main() {
    // Shared State
    // See: https://docs.rs/axum/latest/axum/index.html#sharing-state-with-handlers

    // Holds state of mappings
    let state_of_mappings = Arc::new(AppState {
        berry_m: poke_rs_api::pkmn::mapper::init_mapping("berry"), // berry mappings
        //move_m: poke_rs_api::pkmn::mapper::init_mapping("move"),
        pokemon_m: poke_rs_api::pkmn::mapper::init_mapping("pokemon"),
    });

    // Defaults
    let mut port : u16 = 3001;
    let mut requests_before_limit : u64 = 5;
    let mut max_requests_before_backpressure : usize = 100;
    let mut docker_mode: bool = false;
    
    // COMMAND LINE ARGS
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
        .arg(
            clap::Arg::new("docker")
            .short('d')
            .long("docker")
            .action(clap::ArgAction::SetTrue)
            .help("Turn on docker compatibility mode")
        )
        .get_matches();
    
    // Let user use a custom port
    if let Some(p) = cmd_arg_matches.get_one::<String>("port") {
        
        match p.parse::<u16>() {
            Ok(num) => {port = num;},
            Err(e) => (),
        }
        println!("Value for port is now: {}", port);
    }
    
    if let Some(d) = cmd_arg_matches.get_one::<bool>("docker") {
        if *d == true {
            docker_mode = true;
            println!("enabled docker mode");
        }
    }
    // COMMAND LINE ARGS

    // LOGGING
    //tracing_subscriber::fmt::init()
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    // LOGGING

    //  ROUTES
    let app = Router::new()
        // `GET -> /`
        //.with_state(state)
        .route("/:id", get(poke_rs_api::api::get_id.clone()))
        .route("/", get(get_root))
        .route("/quote", get(return_quote))
        .route("/nanoid", get(poke_rs_api::util::get_nanoid))
        .route("/pokeapi/v2/:_endpoint/:_id", get(pokeapi_endpoint))
        .route("/api/pokeimg/:_endpoint/:_id", get(pokeimg_handler))
        // timeout at 30 seconds
        .layer(
            
            tower::ServiceBuilder::new()
                .layer(axum::error_handling::HandleErrorLayer::new(handle_timeout_error))
                // `cargo add tower --features timeout`
                .timeout(std::time::Duration::from_secs(30))
        )
        .layer(Extension(state_of_mappings))
        // Shared State
        
    ;
    // ROUTES 


    // IP ADDRESSES
    let domain_all = "0.0.0.0";
    // https://crates.io/crates/axum
    // run app
    let localhost = ([127,0,0,1],port);
    let listen_all = ([0,0,0,0], port);
    let mut listen_address = localhost;
    
    if docker_mode {
        listen_address=listen_all;
    }
    
    let addr = SocketAddr::from(listen_address);
    
    tracing::debug!("listening on {}", addr);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    //let listener = tokio::net::TcpListener::bind(format!("{}:{}",domain_all,port));
    //axum::serve(listener, app).await.unwrap();
    
}
// MAIN END

// HANDLERS

// Pokeapi REST api
// Have to allow request to take a Pokemon name and map it to necessary id
#[debug_handler]
async fn pokeapi_endpoint(
        // Note that multiple parametres must be extracted with a tuple Path<(_,_)>
        // State(state): State<std::sync::Arc<AppState>>,
        Extension(state_of_mappings): Extension<Arc<AppState>>,
        axum::extract::Path((_endpoint,_id)): axum::extract::Path<(String,String)>,) -> Result<impl IntoResponse, StatusCode>{
    // START FUNCTION
    let mut is_name = false;
    let og_id=&_id;
    let mut _id2=_id.clone();

    // let cache_location = String::from("pokeapi-cache")

    // TODO Generalize this
    // Works for pokemon only
    // Check to see if id can be parsed as a number if it can, use it directly
    // Else: map it to the pokemon
    match &_id.parse::<u64>() {
        Ok(num) => { println!("Could parse as number")},
        Err(err) => {
            // map to pokemon
            println!("Not a number.");
            is_name=true;
            println!("Got request for name, instead of number");
            // FIX
            // map( name -> id )
            // Generalize this, this is brittle and non-scalable
            match _endpoint.as_str() {
                "berry" => {
                    _id2 = state_of_mappings.berry_m[&_id].clone().as_integer().expect("should be integer").to_string();
                },
                /*"move" => {
                    _id2 = state.move_m[&_id].clone().as_integer().expect("should be integer").to_string();
                },*/
                "pokemon" => {
                    _id2 = state_of_mappings.pokemon_m[&_id].clone().as_integer().expect("should be integer").to_string();
                },
                _ => {
                    println!("invalid endpoint");
                },
            }
            
            println!("mapped: {} -> {}", og_id, _id );
        }
    }

    println!("attempting to get pokeapi/v2: {}/{}", _endpoint, _id2 );

    let file_name = format!("{}.json", _id2);
    // location to look for file
    // Now should run at root
    let file_name_location = format!("pokeapi-cache/{}/{}.json", _endpoint,_id2);
    #[cfg(debug_assertions)]
    {
        println!("file_name_location: {}", file_name_location);
        if !std::path::Path::exists(std::path::Path::new(&file_name_location)) {
            println!("[ERROR]: File not found");
        }
    }
    
    // Read json file to string and use as packet Body
    let contents = match fs::read_to_string(file_name_location) {
        Ok(string) => string,
        Err(err) => err.to_string(),
    };

    // Packet Headers
    let headers = 
    [
        (axum::http::header::CONTENT_TYPE, "text/json; charset=utf-8".to_string()),
    ];
    // Return packet
    Ok((headers, contents))
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

async fn return_quote() -> &'static str {
    r#""Give a man a fire and he's warm for a day, 
but set fire to him and he's warm for the rest of his life."
    - Terry Pratchett, Jingo"#
}

#[debug_handler]
async fn pokeimg_handler(
        // Note that multiple parametres must be extracted with a tuple Path<(_,_)>
        // State(state): State<std::sync::Arc<AppState>>,
        Extension(state_of_mappings): Extension<Arc<AppState>>,
        axum::extract::Path((_endpoint,_id)): axum::extract::Path<(String,String)>,) -> Result<impl IntoResponse, StatusCode>{
    // START FUNCTION
    let mut is_name = false;
    let og_id=&_id;
    let mut _id2=_id.clone();

    // let cache_location = String::from("pokeapi-cache")

    // TODO Generalize this
    // Works for pokemon only
    // Check to see if id can be parsed as a number if it can, use it directly
    // Else: map it to the pokemon
    match &_id.parse::<u64>() {
        Ok(num) => { println!("Could parse as number")},
        Err(err) => {
            // map to pokemon
            println!("Not a number.");
            is_name=true;
            println!("Got request for name, instead of number");
            // FIX
            // map( name -> id )
            // Generalize this, this is brittle and non-scalable
            match _endpoint.as_str() {
                "berry" => {
                    _id2 = state_of_mappings.berry_m[&_id].clone().as_integer().expect("should be integer").to_string();
                    println!("_id2: {}", _id2);
                },
                /*"move" => {
                    _id2 = state.move_m[&_id].clone().as_integer().expect("should be integer").to_string();
                },*/
                "pokemon" => {
                    _id2 = state_of_mappings.pokemon_m[&_id].clone().as_integer().expect("should be integer").to_string();
                    println!("_id2: {}", _id2);

                },
                _ => {
                    println!("invalid endpoint");
                },
            }
            
            println!("mapped: {} -> {}", og_id, _id );
        }
    }

    println!("attempting to get pokeapi/v2: {}/{}", _endpoint, _id2 );

    let file_name = format!("{}.png", _id2);
    // location to look for file
    // Now should run at root
    let file2get = format!("assets/{}/{}.png", _endpoint, _id2);
    #[cfg(debug_assertions)]
    {
        println!("file_name_location: {}", file2get);
        if !std::path::Path::exists(std::path::Path::new(&file2get)) {
            println!("[ERROR]: File not found");
        }
    }
    
    /*
    // Read json file to string and use as packet Body
    let contents = match fs::read_to_string(file_name_location) {
        Ok(string) => string,
        Err(err) => err.to_string(),
    };
    */
    // Repetitive file IO is inefficient, how to improve?
    
    // https://doc.rust-lang.org/std/io/struct.BufReader.html
    // BufReader<R> can improve the speed of programs that make small and repeated read calls to the same file or network socket. 
    // It does not help when reading very large amounts at once, or reading just one or a few times.
    
    // Store in buffer?
    // https://docs.rs/freqfs/0.4.3/freqfs/

    //https://github.com/tokio-rs/axum/discussions/608#discussioncomment-1789020
    let f = match tokio::fs::File::open(file2get).await {
        Ok(file) => file,
        Err(err) => return Err(StatusCode::NOT_FOUND),
    };

    // convert `AsyncRead` to `Stream`
    let stream = ReaderStream::new(f);

    // convert `Stream` into `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    // Packet Headers
    let headers = 
    [
        (axum::http::header::CONTENT_TYPE, "image/png; charset=utf-8".to_string()),
    ];
    // Return packet
    Ok((headers, body))
}
