// HashMaps are OP.
use std::collections::HashMap;
// Calling Warp.
use warp::{Filter, Rejection, Reply};
// Results would be within the parameters below.
type Result<T> = std::result::Result<T, Rejection>;


#[tokio::main]
async fn main() {
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