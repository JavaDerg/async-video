use crate::msg::{Command, Response};
use crate::room::{Room, RoomRequest};
use actix::{Actor, StreamHandler, Addr};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};

pub struct UserCon {
    user_name: String,
    room_handle: Option<Addr<Room>>,
}

impl Default for UserCon {
    fn default() -> Self {
        Self {
            user_name: format!("{}", rand::random::<u32>()), // TODO: Add funny name generator
            room_handle: None,
        }
    }
}

impl Actor for UserCon {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for UserCon {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(msg.as_ref()),
            Ok(ws::Message::Text(json)) => {
                let cmd = match serde_json::from_str::<crate::msg::Command>(&json) {
                    Ok(json) => json,
                    Err(_) => {
                        return respond(
                            ctx,
                            Response::Error(String::from("Received json of questionable quality.")),
                        )
                    }
                };
                match cmd {
                    Command::SetUsername(uname) => {
                        if let Some(room) = &self.room_handle {
                            room.do_send(RoomRequest::UpdateUsername {
                                old: self.user_name.clone(),
                                new: uname.clone(),
                            });
                        }
                        self.user_name = uname;
                    }
                    Command::EnterRoom(room_id) => {}
                }
                todo!()
            }
            _ => (),
        }
    }
}

fn respond(ctx: &mut ws::WebsocketContext<UserCon>, msg: Response) {
    ctx.text(&serde_json::to_string(&msg).unwrap())
}
