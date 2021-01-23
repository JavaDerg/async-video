#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    SetUsername(String),
    EnterRoom(String),
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Response {
    Error(String),
    UsernameUpdate { target: String, new: String },
}
