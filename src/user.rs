use crate::manager::{Manager, ManagerRoomRequest};
use crate::msg::{Command, Response};
use crate::room::{Room, RoomRequest};
use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, ResponseFuture, StreamHandler};
use actix_web_actors::ws;
use actix_web_actors::ws::ProtocolError;

#[derive(Clone)]
pub struct User {
    user_name: String,
    room_handle: Option<Addr<Room>>,
    manager: Addr<Manager>,
}

pub struct UserCon(pub User);

pub struct IncomingMessage(pub Command);

pub struct OutgoingMessage(pub Response);

pub enum UserMutator {
    Username(String),
}

impl User {
    pub fn new(manager: Addr<Manager>) -> Self {
        Self {
            user_name: format!("{}", rand::random::<u32>()), // TODO: Add funny name generator
            room_handle: None,
            manager,
        }
    }
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

impl Message for UserMutator {
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

impl Handler<IncomingMessage> for UserCon {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: IncomingMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg.0 {
            Command::EnterRoom(room_id) => {
                let copy = self.clone();
                Box::pin(async move {
                    let addr = copy
                        .manager
                        .send(ManagerRoomRequest::Query(room_id))
                        .await
                        .unwrap();
                    if let Some(room) = addr {
                        // TODO: Join room
                    } else {
                        // TODO: Create new room
                    }
                })
            }
            Command::SetUsername(uname) => {
                if let Some(room) = &self.0.room_handle {
                    let req = room.send(RoomRequest::UpdateUsername {
                        old: self.0.user_name.clone(),
                        new: uname.clone(),
                    });
                    let this = ctx.address();
                    Box::pin(async move {
                        if req.await.unwrap() {
                            this.send(UserMutator::Username(uname));
                        }
                    })
                } else {
                    self.user_name = uname;
                    Box::pin(async move {})
                }
            }
        }
    }
}

impl Handler<UserMutator> for UserCon {
    type Result = ();

    fn handle(&mut self, msg: UserMutator, _: &mut Self::Context) -> Self::Result {
        match msg {
            UserMutator::Username(uname) => self.0.user_name = uname,
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
