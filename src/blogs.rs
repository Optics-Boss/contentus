pub mod blogs {
    use warp::Filter;
    use crate::blogs::handler;
    use serde::Serialize;
    use serde::Deserialize;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct BlogPost {
        pub id: i32,
        pub titel: String,
        pub content: String,
    }

    pub fn blogposts() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        blogposts_list()
            .or(index())
    }

    pub fn index() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!()
            .and(warp::get())
            .map(|| "Contentus")
    }

    pub fn blogposts_list() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {

        warp::path!("blogs")
            .and(warp::get())
            .and(&handler::list_blogs())
    }


}

mod handler {
    use std::convert::Infallible;
    use crate::blogs::blogs::BlogPost;

    pub fn list_blogs() -> Result<impl warp::Reply, Infallible> {
        let blogs : Vec<BlogPost> = vec![
            BlogPost{ 
                id: 1, 
                titel: String::from("Test"), 
                content: String::from("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.") 
            },
            BlogPost{ 
                id: 2,
                titel: String::from("Test 2"),
                content: String::from("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")
            }
        ];

        Ok(warp::reply::json(&blogs))
    }
}
