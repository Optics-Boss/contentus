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
            .and_then(handler::list_blogs)
    }
}

mod handler {
    use std::{convert::Infallible, fs::File, vec};
    use csv::StringRecord;

    use crate::blogs::blogs::BlogPost;

    pub async fn list_blogs() -> Result<impl warp::Reply, Infallible> {

        let file = File::open("database/test.csv").expect("Can't open file");
        let mut reader = csv::Reader::from_reader(file);
        let mut blogs : Vec<BlogPost> = vec!();

        for (index, line) in reader.records().into_iter().enumerate() {
            let record = StringRecord::from(line.unwrap());
            let titel = record.get(0); 
            let content = record.get(1); 

            blogs.push(
                BlogPost { 
                    id: (index + 1) as i32, 
                    titel: String::from(titel.unwrap_or("")), 
                    content: String::from(content.unwrap_or("")) 
                }
            );
        }

        Ok(warp::reply::json(&blogs))
    }
}
