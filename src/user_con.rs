use crate::manager::{Manager, ManagerRoomRequest};
use crate::msg::{Command, Response};
use crate::room::{Room, RoomRequest};
use actix::{Actor, Addr, Context, Handler, Message, StreamHandler, ResponseFuture};
use actix_web_actors::ws;
use actix_web_actors::ws::ProtocolError;

#[derive(Clone)]
pub struct User {
    user_name: String,
    room_handle: Option<Addr<Room>>,
    manager: Addr<Manager>,
}

pub struct UserCon(Addr<User>);

pub struct IncomingMessage(Command);

pub struct OutgoingMessage(Response);

impl User {
    pub fn new(manager: Addr<Manager>) -> Self {
        Self {
            user_name: format!("{}", rand::random::<u32>()), // TODO: Add funny name generator
            room_handle: None,
            manager,
        }
    }
}

impl Actor for User {
    type Context = Context<Self>;
}

impl Actor for UserCon {
    type Context = ws::WebsocketContext<Self>;
}

impl Message for IncomingMessage {
    type Result = ();
}

impl Message for OutgoingMessage {
    type Result = ();
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for UserCon {
    fn handle(&mut self, msg: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(msg.as_ref()),
            Ok(ws::Message::Text(json)) => {
                let cmd = match serde_json::from_str::<crate::msg::Command>(&json) {
                    Ok(json) => json,
                    Err(_) => {
                        return ctx.text(
                            &serde_json::to_string(&Response::Error(String::from(
                                "Received json of questionable quality.",
                            )))
                                .unwrap(),
                        );
                    }
                };
            }
            _ => (),
        }
    }
}

impl Handler<IncomingMessage> for User {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: IncomingMessage, _ctx: &mut Self::Context) -> Self::Result {
        match msg.0 {
            Command::EnterRoom(room_id) => {
                let copy = self.clone();
                Box::pin(async move {
                    let lel = copy.manager.send(ManagerRoomRequest::Query(room_id)).await.unwrap();
                    if let Some(room) = lel {
                        // TODO: Join room
                    } else {
                        // TODO: Create new room
                    }
                })
            }
            Command::SetUsername(uname) => {
                if let Some(room) = &self.room_handle {
                    room.do_send(RoomRequest::UpdateUsername {
                        old: self.user_name.clone(),
                        new: uname.clone(),
                    });
                }
                self.user_name = uname;
                Box::pin(async move {})
            }
        }
    }
}

impl Handler<OutgoingMessage> for UserCon {
    type Result = ();

    fn handle(
        &mut self,
        msg: OutgoingMessage,
        ctx: &mut ws::WebsocketContext<UserCon>,
    ) -> Self::Result {
        ctx.text(&serde_json::to_string(&msg.0).unwrap())
    }
}
