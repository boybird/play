use actix_web::{http, middleware, server, App};

mod routes;
fn main() {
    server::new(|| {
        App::new()
             .middleware(middleware::Logger::default())
            .configure(routes::config)
        //.resource("/",|r| r.method(http::Method::GET).f(routes::index))
        //.resource("blog", |r| r.method(http::Method::GET).f(routes::blog))
    })
    .bind("127.0.0.1:8079")
    .unwrap().run();
    
}
