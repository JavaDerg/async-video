use std::ops::Deref;
use std::sync::{Arc, Weak};

pub struct Sender<T: Clone>(
    flume::Receiver<flume::Sender<T>>,
    Vec<Option<flume::Sender<T>>>,
);

pub struct Receiver<T>(AoW<flume::Sender<flume::Sender<T>>>, flume::Receiver<T>);

#[derive(Clone)]
pub enum AoW<T> {
    Arc(Arc<T>),
    Weak(Weak<T>),
}

impl<T> AoW<T> {
    pub fn upgrade(&mut self) -> bool {
        let arc = if let Self::Weak(weak) = &self {
            weak.upgrade()
        } else {
            return true;
        };
        if arc.is_none() {
            return false;
        }
        *self = Self::Arc(arc.unwrap());
        true
    }

    pub fn downgrade(&mut self) {
        let weak = match self {
            Self::Arc(arc) => {
                Arc::downgrade(arc)
            }
            Self::Weak(_) => return,
        };
        *self = Self::Weak(weak);
    }
}

impl<T: Clone> Sender<T> {
    pub fn send(&mut self, t: T) -> Result<(), flume::SendError<T>> {
        if self.0.is_disconnected() {
            return Err(flume::SendError(t));
        }
        for nr in self.0.drain() {
            self.1.push(Some(nr));
        }
        for re in self.1.iter_mut().filter(|re| re.is_some()) {
            let r = re.as_mut().unwrap();
            if let Err(_) = r.send(t.clone()) {
                *re = None;
            }
        }
        Ok(())
    }

    pub async fn send_async(&mut self, t: T) -> Result<(), flume::SendError<T>> {
        if self.0.is_disconnected() {
            return Err(flume::SendError(t));
        }
        for nr in self.0.drain() {
            self.1.push(Some(nr));
        }
        for re in self.1.iter_mut().filter(|re| re.is_some()) {
            let r = re.as_mut().unwrap();
            if let Err(_) = r.send_async(t.clone()).await {
                *re = None;
            }
        }
        Ok(())
    }
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> Result<T, flume::RecvError> {
        self.1.recv()
    }

    pub async fn recv_async(&self) -> Result<T, flume::RecvError> {
        self.1.recv_async().await
    }

    pub fn downgrade(&mut self) {
        self.0.downgrade();
    }

    pub fn upgrade(&mut self) -> bool {
        self.0.upgrade()
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        let (tx, rx) = flume::unbounded();
        match &self.0 {
            AoW::Arc(atx) => atx.send(tx),
            AoW::Weak(wtx) => {
                if let Some(atx) = wtx.upgrade() {
                    atx.send(tx)
                }
            }
        };
        Self(self.0.clone(), rx)
    }
}

pub fn unbounded<T: Clone>() -> (Sender<T>, Receiver<T>) {
    let (fb_tx, fb_rx) = flume::unbounded();
    let (data_tx, data_rx) = flume::unbounded();
    (Sender(fb_rx, vec![Some(data_tx)]), Receiver(AoW::Arc(Arc::new(fb_tx)), data_rx))
}


#[cfg(test)]
#[test]
fn broadcast_test() {
    let (mut tx, rx) = unbounded::<u32>();
    let rx2 = rx.clone();
    tx.send(0xDEADBEEF);
    assert_eq!(rx.recv(), Ok(0xDEADBEEF));
    assert_eq!(rx2.recv(), Ok(0xDEADBEEF));
    let rx3 = rx.clone();
    tx.send(0x1337FFFF);
    assert_eq!(rx.recv(), Ok(0x1337FFFF));
    assert_eq!(rx2.recv(), Ok(0x1337FFFF));
    assert_eq!(rx3.recv(), Ok(0x1337FFFF));
    drop(tx);
    assert_eq!(rx.recv(), Err(flume::RecvError::Disconnected));
    assert_eq!(rx2.recv(), Err(flume::RecvError::Disconnected));
}
