use crate::room::RoomRef;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct RoomManager(Arc<tokio::sync::RwLock<HashMap<Uuid, RoomRef>>>);
