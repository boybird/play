use actix::{Actor, StreamHandler};
use actix_web::{server, ws, App};
#[macro_use]
extern crate failure;

struct Room {
    name: String,
}
#[derive(Debug, Fail)]
enum SystemError {
    #[fail(display = "ws:{}", _0)]
    WsProtocolErr(#[fail(cause)] ws::ProtocolError),
    #[fail(display = "io:{}", _0)]
    IoError(#[fail(cause)] std::io::Error),
}

impl std::convert::From<std::io::Error> for SystemError {
    fn from(e: std::io::Error) -> SystemError {
        SystemError::IoError(e)
    }
}

struct Ws {
    rooms: Vec<Room>,
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("{:?}", ctx.state());
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            _ => (),
        }
    }
}

fn main() -> Result<(), SystemError> {
    
    server::new(|| {
        App::new().resource("/ws/{url}", |r| {
            
            r.f(|req| {
                println!("{:?}", req);
                ws::start(req, Ws { rooms: vec![] })
            })
        })
    })
    .bind("0.0.0.0:1257")?
    .run();

    Ok(())
}
