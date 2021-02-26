mod broadcast;
mod handler;
mod room;

use std::net::SocketAddr;
use warp::Filter;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Assets;

#[tokio::main]
async fn main() {
    let assets = Assets;
    let assets = warp_embed::embed(&assets).or(warp_embed::embed_one(&assets, "index.html"));

    let addr = std::env::args()
        .skip(1)
        .map(|addr| {
            addr.parse::<SocketAddr>()
                .expect(&format!("'{}' is not a valid socket address", addr))
        })
        .take(1)
        .next()
        .expect("No socket address supplies");

    let handler = warp::path("gen")
        .and(warp::get())
        .and_then(handler::gen_id)
        .or(assets);

    warp::serve(handler).run(addr).await;
}
