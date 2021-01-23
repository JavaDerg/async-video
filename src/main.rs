use actix_web::error::Error;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod manager;
mod msg;
mod room;
mod user_con;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind(std::env::var("INTERFACE").unwrap_or_else(|_| String::from("0.0.0.0:8080")))?
        .run()
        .await
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(user_con::UserCon::default(), &req, stream);
    println!("{:?}", resp);
    resp
}
