use std::task::{Context, Poll};
use std::pin::Pin;
use tokio_stream::Stream;
use std::time::{Instant, Duration};
use uuid::Uuid;
use std::collections::HashMap;

pub struct Room {
    video_url: String,
    playlist: Vec<(String, Uuid)>,
    // State
    playing_since: Instant,
    paused_since: Option<Instant>,
    pause_subtraction: Duration,
    // Members
    members: HashMap<Uuid, String>,
}

#[serde(serde::Serialize)]
pub enum StateUpdate {
    CurrentPosition
}

impl Room {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            video_url: String::from("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            playlist: vec![],
            playing_since: now,
            paused_since: Some(now),
            pause_subtraction: now - now,
            members: HashMap::default(),
        }
    }

    pub fn time(&self) -> Duration {
        let mut dur = Instant::now() - self.playing_since - self.pause_subtraction;
        if Some(pause) = self.paused_since.as_ref() {
            dur -= pause;
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
    }

    pub fn pause(&mut self) {
        if self.paused_since.is_some() {
            return;
            self.paused_since = Some(Instant::now());
        }
    }
}

impl Stream for Room {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unimplemented!()
    }
}
