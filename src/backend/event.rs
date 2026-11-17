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

pub struct Outgoing {
    pub chat: String,
    pub text: String,
}

static SINK: OnceLock<UnboundedSender<Event>> = OnceLock::new();
static OUT: OnceLock<UnboundedSender<Outgoing>> = OnceLock::new();

pub fn channel() -> Option<(UnboundedReceiver<Event>, UnboundedReceiver<Outgoing>)> {
    let (sink, events) = unbounded_channel();
    let (out, outbox) = unbounded_channel();
    SINK.set(sink).ok()?;
    OUT.set(out).ok()?;
    Some((events, outbox))
}

pub fn send(chat: &str, text: &str) {
    if let Some(out) = OUT.get() {
        out.send(Outgoing { chat: chat.into(), text: text.into() }).ok();
    }
}

pub fn emit(event: Event) {
    if let Some(sink) = SINK.get() {
        sink.send(event).ok();
    }
}
