use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::Stream;
use tokio::sync::mpsc::UnboundedReceiver;

use super::events::FileEvent;

#[derive(Debug)]
pub struct ScannerEventStream {
    receiver: UnboundedReceiver<FileEvent>,
}

impl ScannerEventStream {
    pub fn new(receiver: UnboundedReceiver<FileEvent>) -> Self {
        Self { receiver }
    }
}

impl Stream for ScannerEventStream {
    type Item = FileEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.receiver.poll_recv(cx) {
            Poll::Ready(event) => Poll::Ready(event),
            Poll::Pending => Poll::Pending,
        }
    }
}
