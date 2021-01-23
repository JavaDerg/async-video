use crate::manager::Manager;
use actix::{Actor, Addr};
use actix_web::error::Error;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod manager;
mod msg;
mod room;
mod user_con;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(index).data(Manager::default().start()))
        .bind(std::env::var("INTERFACE").unwrap_or_else(|_| String::from("0.0.0.0:8080")))?
        .run()
        .await
}

#[actix_web::get("/ws/")]
async fn index(
    req: HttpRequest,
    stream: web::Payload,
    manager: web::Data<Addr<Manager>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(user_con::UserCon::new((**manager).clone()), &req, stream);
    println!("{:?}", resp);
    resp
}
