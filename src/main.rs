use actix_web::{http, middleware, server, App};

mod routes;
fn main2() {
    server::new(|| {
        App::new()
            .middleware(middleware::Logger::default())
            .configure(routes::config)
        //.resource("/",|r| r.method(http::Method::GET).f(routes::index))
        //.resource("blog", |r| r.method(http::Method::GET).f(routes::blog))
    })
    .bind("127.0.0.1:8079")
    .unwrap()
    .run();
}

//extern crate serde;
//extern crate serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct MyMap {
    #[serde(rename = "innerMap")]
    inner_map: HashMap<i32, String>,
}

#[macro_use]
extern crate failure;
#[derive(Debug, Fail)]
enum MainError {
    #[fail(display = "serde error:{}", _0)]
    SerdeError(#[fail(cause)] serde_json::Error),
}

impl std::convert::From<serde_json::Error> for MainError {
    fn from(e: serde_json::Error) -> MainError {
        MainError::SerdeError(e)
    }
}

fn main() -> Result<(), MainError> {
    let c = (0..3).map(|i| i + 1).collect::<Vec<i32>>();
    let m = (0..3)
        .enumerate()
        .map(|(u, i)| (u, i.to_string()))
        .collect::<HashMap<usize, String>>();
    println!("{:?}", c);
    println!("{:?}", m);
    let mymap = MyMap {
        inner_map: (0..3)
            .enumerate()
            .map(|(u, i)| (u as i32, i.to_string()))
            .collect(),
    };
    println!("{:?}", mymap);
    let s = serde_json::to_string(&mymap)?;
    println!("{}", s);
    //let s = s.trim_end_matches("}");
    let b: MyMap = serde_json::from_str(&s)?;
    println!("{:?}", mymap);

    Ok(())
}
