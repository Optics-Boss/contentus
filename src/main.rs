#![deny(warnings)]
use warp::Filter;

mod blogs;
use blogs::blogs::blogposts;

#[tokio::main]
async fn main() { 
    let api = blogposts();
    let routes = api.with(warp::log("contentus"));

    warp::serve(routes).run(([127, 0 ,0 ,1], 3030)).await;
}
