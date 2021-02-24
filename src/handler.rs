use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub async fn gen_id() -> Result<impl Reply, Rejection> {
    Ok(crate::room::gen_id())
}

pub async fn enter_room(ws: warp::ws::Ws, room_id: String) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(|socket| async move { handle_stream(socket).await; }))
}

pub async fn handle_stream(ws: warp::ws::WebSocket) {
    // let (sink, stream) = ws.boxed_local().split();
}
