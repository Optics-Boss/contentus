pub mod blogs {
    use warp::Filter;

    pub fn blogposts() -> impl Filter<Extract = (impl warp::Reply,),
    Error = warp::Rejection> + Clone {
        blogposts_list()
            .or(index())
    }

    pub fn index() -> impl Filter<Extract = (impl warp::Reply,), 
    Error = warp::Rejection> + Clone {
        warp::path!()
            .and(warp::get())
            .map(|| "Contentus")
    }

    pub fn blogposts_list() -> impl Filter<Extract = (impl warp::Reply,), 
    Error = warp::Rejection> + Clone {
        warp::path!("blogs")
            .and(warp::get())
            .map(|| "Hello, World!")
    }

}
