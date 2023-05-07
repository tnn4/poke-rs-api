#![cfg_attr(debug_assertions, allow(unused_imports))]

// use std::path::Path;

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

#[derive(Clone,Serialize, Deserialize)]
struct AppState {
    m: toml::Table,
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
    let state = Arc::new(AppState {
        m: init_pokemap(),
    });

    // Defaults
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
    
    // Let user use a custom port
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

    // Generate ROUTES
    let app = Router::new()
        // `GET -> /`
        //.with_state(state)
        .route("/:id", get_id_route.clone())
        .route("/", get(get_root))
        .route("/quote", get(return_quote))
        .route("/toml", get(return_toml_handler))
        .route("/nanoid", get(poke_rs_api::util::get_nanoid))
        .route("/pokeapi/v2/:_endpoint/:_id", get(pokeapi_handler_for_id))
        // timeout at 30 seconds
        .layer(
            
            tower::ServiceBuilder::new()
                .layer(axum::error_handling::HandleErrorLayer::new(handle_timeout_error))
                // `cargo add tower --features timeout`
                .timeout(std::time::Duration::from_secs(30))
        )
        .layer(Extension(state))
        // Shared State
        
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

// Handlers
// Pokeapi REST api
// Have to allow request to take a string and map it to necessary id
#[debug_handler]
async fn pokeapi_handler_for_id(
        // Note that multiple parametres must be extracted with a tuple Path<(_,_)>
        // State(state): State<std::sync::Arc<AppState>>,
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path((_endpoint,_id)): axum::extract::Path<(String,String)>,
        
        
    )
    // axum::extract::Path(_id): axum::extract::Path<u32>) 
    -> Result<impl IntoResponse, StatusCode>{
    
    let mut is_string = false;
    let og_id=&_id;
    // let cache_location = String::from("pokeapi-cache")

    // Check to see if id can be parsed as a number if it can, use it directly
    // Else: map it to the pokemon
    match &_id.parse::<u64>() {
        Ok(num) => { println!("Could parse as number")},
        Err(err) => {
            // map to pokemon
            println!("Not a number.");
            is_string=true;
        }
    }
    
    if is_string {
        
        println!("Got request for pokemon, instead of number");
        // map name -> id
        let _id = &state.m[&_id]; // ERROR here
        println!("mapped: {} -> {}", og_id, _id );
    }

    println!("attempting to get pokeapi/v2: {}/{}", _endpoint, _id );

    let file_name = format!("{}.json", _id);
    // location to look for file
    
    
    let file_name_location = format!("../pokeapi-cache/{}/{}.json", _endpoint,_id);
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

    Ok((headers, contents))
}

// return file
async fn return_json_file_berry(
    axum::extract::Path(id): axum::extract::Path<u32>)-> Result<impl IntoResponse,StatusCode>{
    
    // filenames
    let file_name = format!("{}.json",id);
    // location to look for file
    let file_name_path = format!("../berry/{}.json",id);

    // open the file
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

    // Create body for content header
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

fn init_pokemap() -> /*HashMap<String,String>*/toml::Table {
    let file_path="../poke2id.toml";
    let mut toml_object: toml::Table = Default::default();
    assert!(std::path::Path::new(file_path).exists());
    
    // open toml
    //let mut file=std::fs::File::open(file_path);
    let mut contents = String::new();
    // WARNING: the variable inside if let is a totally different variable
    if let contents2=std::fs::read_to_string(file_path){
        println!("READ: success");
        // println!("{}", contents.unwrap());
        contents = contents2.unwrap();
    } else {
        panic!("[ERROR]: can't read file");
    }
    //match std::fs::read_to_string(&mut toml) {
        //Ok(string) => string,
        //Err(err) => err,
    //};
    println!("{}", contents);
    // let toml_object : HashMap<String, String> = toml::from_str(&contents).unwrap();
    
    if let toml_object2 = contents.parse::<toml::Table>().unwrap() {
        println!("[OK]:POKEMAP initialized ");
        toml_object=toml_object2;
    } else {
        panic!("[ERROR]: could not read mapping");
    }
    
    // ERROR here
    println!("toml_object: {:?}", toml_object); // -> {}
    // assert_eq!(toml_object["bulbasaur"], "1".to_string());
    // assert_eq!(toml_object["bulbasaur"], toml::Value::String("1".to_string()));
    assert_eq!(toml_object["bulbasaur"], toml::Value::Integer(1));
    println!("toml[bulbasaur]: {}", toml_object["bulbasaur"]);
    // read it
    // then add it to dictionary
    // let mut poke2id: HashMap<String,String> = HashMap::new();
    toml_object
}