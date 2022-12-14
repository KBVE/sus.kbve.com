// FS - Read Environmental Variables from Docker Secrets
use std::fs;

//
use std::env;

//
use std::sync::Once;

// FS Async
//use tokio::fs;


// HashMaps are OP.
//  use std::collections::HashMap;

// Headers
//  use hyper::header::{Headers, Authorization, Bearer};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};

// Rocket
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;

// Calling Warp.
use warp::{Filter, Rejection, Reply};
// Results would be within the parameters below.
type Result<T> = std::result::Result<T, Rejection>;

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers
}

fn setenv() {
    let file_path = "/run/secrets/API_TOKEN_FILE";
    let api_token = fs::read_to_string(file_path)
    .expect("API TOKEN FILE is missing");
    env::set_var("API_TOKEN", api_token);
    println!("Env Set!");
}



#[tokio::main]
async fn main() {

     // Initialization of ENV
    static START: Once = Once::new();
    START.call_once(|| {
       
            setenv();
        });

    // API Token - We need to replace the current method.    
    let api_token = env::var("API_TOKEN").unwrap_or("Test".to_string()).as_str().to_owned();


    let client = Client::new();
    let resp = client
    .post("https://api.kbve.com/api/")
    .headers(construct_headers())
    .bearer_auth(api_token)
    .send();

    


    // Init. Hello wrap.
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let health_route = warp::path!("health").and_then(health_check);

    // Final Routes <GET>
    let get_routes =
        warp::
            get()
            .and(hello);


    let any_origin_routes = 
        warp::
            get()
            .and(health_route)
            .with(warp::cors().allow_any_origin());
    
    let routes = 
        get_routes.or(any_origin_routes);
    
    // Init Host variables
    let (host , port) = ([0,0,0,0], 3030);

    println!("Started the init server on: {}:{}", host.map(|a| a.to_string()).join("."), port);
    warp::serve(routes)
        .run((host, port))
        .await;

}

async fn health_check() -> Result<impl Reply> {
    Ok("OK")
}