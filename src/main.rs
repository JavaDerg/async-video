mod room;

use warp::Filter;
use std::net::SocketAddr;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Assets;

#[tokio::main]
async fn main() {
    let assets = Assets;
    let assets = warp_embed::embed(&assets)
        .or(warp_embed::embed_one(&assets, "index.html"));

    let listener = std::env::args()
        .skip(1)
        .map(|addr| addr.parse::<SocketAddr>().expect(&format!("'{}' is not a valid socket address", addr)));

    warp::serve(asserts)
        .run(([0, 0, 0, 0], 8080))
        .await;
}
