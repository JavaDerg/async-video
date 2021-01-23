use crate::room::Room;
use actix::{Actor, Addr, Context, Handler, Message};
use std::collections::HashMap;

#[derive(Default)]
pub struct Manager(HashMap<String, Addr<Room>>);

pub enum ManagerRoomRequest {
    Query(String),
    Create(String, Addr<Room>),
    Delete(String),
}

impl Actor for Manager {
    type Context = Context<Self>;
}

impl Message for ManagerRoomRequest {
    type Result = Option<Addr<Room>>;
}

impl Handler<ManagerRoomRequest> for Manager {
    type Result = Option<Addr<Room>>;

    fn handle(&mut self, msg: ManagerRoomRequest, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ManagerRoomRequest::Query(query) => self.0.get(&query).map(Addr::clone),
            ManagerRoomRequest::Create(query, room) => {
                let _ = self.0.insert(query, room.clone());
                Some(room)
            }
            ManagerRoomRequest::Delete(query) => self.0.remove(&query),
        }
    }
}
