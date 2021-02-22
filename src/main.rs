use warp::Filter;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Assets;

#[tokio::main]
async fn main() {
    let assets = Assets;
    let assets = warp_embed::embed(&assets)
        .or(warp_embed::embed_one(&assets, "index.html"));

    warp::serve(assets)
        .run(([0, 0, 0, 0], 8080))
        .await;
}
