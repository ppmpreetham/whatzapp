// for first qr, bro needs to login thru his phone

use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::UnboundedReceiver;

use super::bot::{self, Exit};
use super::event::{self, emit, Event};
use super::store;

const MAX_WAIT: u64 = 30;

pub fn start() -> Option<UnboundedReceiver<Event>> {
    let events = event::channel()?;
    std::thread::spawn(supervise);
    Some(events)
}

fn supervise() {
    let Ok(r) = Runtime::new() else {
        return emit(Event::Failed("no async runtime".into()));
    };
    let mut wait = 1;
    loop {
        match r.block_on(bot::run()) {
            Exit::LoggedOut => {
                store::forget();
                emit(Event::LoggedOut);
                wait = 1;
            }
            Exit::Dropped => {
                emit(Event::Offline);
                std::thread::sleep(Duration::from_secs(wait));
                wait = (wait * 2).min(MAX_WAIT);
            }
            Exit::Failed(reason) => {
                emit(Event::Failed(reason));
                std::thread::sleep(Duration::from_secs(wait));
                wait = (wait * 2).min(MAX_WAIT);
            }
        }
    }
}
