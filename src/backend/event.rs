use std::sync::OnceLock;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub enum Event {
    Qr(String),
    Connected,
    LoggedOut,
    Offline,
    Failed(String),
    Message { chat: String, mine: bool, text: String },
}

static SINK: OnceLock<UnboundedSender<Event>> = OnceLock::new();

pub fn channel() -> Option<UnboundedReceiver<Event>> {
    let (sink, events) = unbounded_channel();
    SINK.set(sink).ok()?;
    Some(events)
}

pub fn emit(event: Event) {
    if let Some(sink) = SINK.get() {
        sink.send(event).ok();
    }
}
