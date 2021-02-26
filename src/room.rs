use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Weak;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use crate::broadcast::{Sender, Receiver};
use rand::Rng;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_stream::Stream;
use uuid::Uuid;

mod manager;

pub struct Room {
    id: String,
    // Queue
    video_index: u32,
    playlist: Vec<(String, Uuid)>,
    // State
    playing_since: Instant,
    paused_since: Option<Instant>,
    pause_subtraction: Duration,
    // Members
    members: HashMap<Uuid, String>,
    // Stream
    broadcaster: Sender<StateUpdate>,
    receiver: Receiver<StateUpdate>
}

pub struct RoomRef(Weak<RwLock<Room>>);

#[derive(serde::Serialize)]
pub enum StateUpdate {
    CurrentPosition,
}

impl Room {
    pub fn new<S: Into<String>>(id: S) -> (Self, Receiver<StateUpdate>) {
        let now = Instant::now();
        let (tx, rx) = crate::broadcast::unbounded();
        let mut lw = rx.clone();
        lw.downgrade();
        (Self {
            id: id.into(),
            video_index: 0,
            playlist: vec![(
                String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
                Uuid::nil(),
            )],
            playing_since: now,
            paused_since: Some(now),
            pause_subtraction: now - now,
            members: HashMap::default(),
            broadcaster: tx,
            receiver: lw
        }, rx)
    }

    pub fn time(&self) -> Duration {
        let mut dur = Instant::now() - self.playing_since - self.pause_subtraction;
        if let Some(pause) = self.paused_since.as_ref() {
            dur -= Instant::now() - *pause;
        }
        dur
    }

    pub fn play(&mut self) {
        if self.paused_since.is_none() {
            return;
        }
        let start = self.paused_since.take().unwrap(); // this always should be some in this state
        let end = Instant::now() - start;
        self.pause_subtraction += end;
        // TODO: Update state
    }

    pub fn pause(&mut self) {
        if self.paused_since.is_some() {
            return;
            self.paused_since = Some(Instant::now());
        }
        // TODO: Update state
    }

    pub fn next(&mut self) {
        self.playing_since = Instant::now();
        self.pause();
        if self.video_index < self.playlist.len() as u32 {
            self.video_index += 1;
        }
        // TODO: Update state
    }
}

lazy_static::lazy_static! {
    static ref WORDS: &'static [&'static str] =
        include_str!("../bips/bip-0039/english.txt")
            .lines()
            .collect::<Vec<_>>()
            .leak();
}

pub fn gen_id() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{}-{}-{}",
        WORDS[rng.gen_range(0..WORDS.len())],
        WORDS[rng.gen_range(0..WORDS.len())],
        WORDS[rng.gen_range(0..WORDS.len())]
    )
}
