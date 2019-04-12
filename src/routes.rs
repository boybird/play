use actix_web::{Responder,HttpRequest,http,App};


pub fn config(app : App) -> App {

        

         app.resource("/",|r| r.method(http::Method::GET).f(index))
        .resource("blog", |r| r.method(http::Method::GET).f(blog))   
}

pub fn index(_r:& HttpRequest)->impl Responder{
    "123".to_string()
}



pub fn blog(_r:& HttpRequest)->impl Responder{
    "123".to_string()
}