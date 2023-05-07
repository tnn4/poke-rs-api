// api::pokeapi

// Handlers
// Pokeapi REST api
// Have to allow request to take a string and map it to necessary id
#[debug_handler]
async fn pokemon_endpoint(
        // Note that multiple parametres must be extracted with a tuple Path<(_,_)>
        // State(state): State<std::sync::Arc<AppState>>,
        Extension(state): Extension<Arc<AppState>>,
        axum::extract::Path((_endpoint,_id)): axum::extract::Path<(String,String)>,    
    )
    // axum::extract::Path(_id): axum::extract::Path<u32>) 
    -> Result<impl IntoResponse, StatusCode>{
    
    let mut is_name = false;
    let og_id=&_id;
    let mut _id2=_id.clone();
    // let cache_location = String::from("pokeapi-cache")

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
            // map name -> id
            _id2 = state.m[&_id].clone().as_integer().expect("should be integer").to_string(); // ERROR here
            println!("mapped: {} -> {}", og_id, _id );
        }
    }

    println!("attempting to get pokeapi/v2: {}/{}", _endpoint, _id2 );

    let file_name = format!("{}.json", _id2);
    // location to look for file
    let file_name_location = format!("../pokeapi-cache/{}/{}.json", _endpoint,_id2);
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