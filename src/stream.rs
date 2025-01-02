use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::Stream;
use tokio::sync::mpsc::Receiver;

use crate::event::MonitorEvent;

pub struct MonitorStream {
    rx: Receiver<MonitorEvent>,
}

impl Stream for MonitorStream {
    type Item = MonitorEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}
