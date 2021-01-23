use crate::manager::Manager;
use actix::{Actor, Addr, Context, Message, Handler};
use crate::user_con::UserCon;
use std::collections::HashMap;

pub struct Room {
    manager: Addr<Manager>,
    id: String,
    user: HashMap<String, Addr<UserCon>>,
}

pub enum RoomRequest {
    JoinRoom(String),
    UpdateUsername {
        old: String,
        new: String,
    }
}

impl Actor for Room {
    type Context = Context<Self>;

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.manager
            .do_send(crate::manager::RoomRequest::Delete(self.id.clone()));
    }
}

impl Message for RoomRequest {
    type Result = bool;
}

impl Handler<RoomRequest> for Room {
    type Result = bool;

    fn handle(&mut self, msg: RoomRequest, ctx: &mut Context<Self>) -> Self::Result {
        unimplemented!()
    }
}
