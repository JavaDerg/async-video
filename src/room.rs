use crate::manager::Manager;
use crate::msg::Response;
use crate::user::{User, UserCon, OutgoingMessage};
use actix::{Actor, Addr, Context, Handler, Message};
use std::collections::HashMap;

pub struct Room {
    manager: Addr<Manager>,
    id: String,
    users: HashMap<String, Addr<UserCon>>,
}

pub enum RoomRequest {
    JoinRoom(String, Addr<User>),
    UpdateUsername { old: String, new: String },
}

impl Actor for Room {
    type Context = Context<Self>;

    fn stopped(&mut self, _: &mut Self::Context) {
        self.manager
            .do_send(crate::manager::ManagerRoomRequest::Delete(self.id.clone()));
    }
}

impl Message for RoomRequest {
    type Result = bool;
}

impl Handler<RoomRequest> for Room {
    type Result = bool;

    fn handle(&mut self, msg: RoomRequest, _: &mut Context<Self>) -> Self::Result {
        match msg {
            RoomRequest::JoinRoom(user, handle) => {}
            RoomRequest::UpdateUsername { old, new } => {
                if self.users.contains_key(&new) || !self.users.contains_key(&old) {
                    return false;
                }
                let user = self.users.remove(&old).unwrap();
                let _ = self.users.insert(new.clone(), user);
                let update = Response::UsernameUpdate { target: old, new };
                for user in self.users.values() {
                    user.do_send(OutgoingMessage(update.clone()));
                }
            }
        }
        unimplemented!()
    }
}
