use crate::room::Room;
use actix::{Actor, Addr, Context, Handler, Message};
use std::collections::HashMap;

pub struct Manager(HashMap<String, Addr<Room>>);

pub enum RoomRequest {
    Query(String),
    Create(String, Addr<Room>),
    Delete(String),
}

impl Actor for Manager {
    type Context = Context<Self>;
}

impl Message for RoomRequest {
    type Result = Option<Addr<Room>>;
}

impl Handler<RoomRequest> for Manager {
    type Result = Option<Addr<Room>>;

    fn handle(&mut self, msg: RoomRequest, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            RoomRequest::Query(query) => self.0.get(&query).map(Addr::clone),
            RoomRequest::Create(query, room) => {
                let _ = self.0.insert(query, room.clone());
                Some(room)
            }
            RoomRequest::Delete(query) => self.0.remove(&query),
        }
    }
}
