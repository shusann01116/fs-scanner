use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::Stream;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::events::ScanEvent;

#[derive(Debug)]
pub struct ScannerEventStream {
    receiver: UnboundedReceiver<ScanEvent>,
}

impl ScannerEventStream {
    pub fn new(receiver: UnboundedReceiver<ScanEvent>) -> Self {
        Self { receiver }
    }
}

impl Stream for ScannerEventStream {
    type Item = ScanEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.receiver.poll_recv(cx) {
            Poll::Ready(event) => Poll::Ready(event),
            Poll::Pending => Poll::Pending,
        }
    }
}
