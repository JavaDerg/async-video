pub struct Sender<T : Clone>(flume::Receiver<flume::Sender<T>>, Vec<Option<flume::Sender<T>>>);

pub struct Receiver<T>(flume::Sender<flume::Sender<T>>, flume::Receiver<T>);

impl<T : Clone> Sender<T> {
    pub async fn send(&mut self, t: T) -> Result<(), flume::SendError<T>> {
        if self.0.is_disconnected() {
            return Err(flume::SendError(t))
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
    pub async fn recv(&self) -> Result<T, flume::RecvError> {
        self.1.recv_async().await
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        let (tx, rx) = flume::unbounded();
        self.0.send(tx);
        Self(self.0.clone(), rx)
    }
}
