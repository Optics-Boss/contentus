#![deny(warnings)]
use warp::Filter;

use serde::Serialize;
use serde::Deserialize;

mod blogs;
use blogs::blogs::blogposts;

#[derive(Serialize, Deserialize)]
struct BlogPost {
    id: i32,
    titel: i32,
    content: i32,
}


#[tokio::main]
async fn main() { 
    let api = blogposts();
    let routes = api.with(warp::log("contentus"));

    warp::serve(routes).run(([127, 0 ,0 ,1], 3030)).await;
}
