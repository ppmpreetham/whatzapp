use std::sync::atomic::{AtomicBool, Ordering};
use whatsapp_rust::prelude::*;
use whatsapp_rust::types::events::ConnectFailureReason;
use whatsapp_rust::{ProtocolTerminalReason, RunCompletionReason};

use super::event::{emit, Event};
use super::store;

pub enum Exit {
    LoggedOut,
    Dropped,
    Failed(String),
}

static LOGGED_OUT: AtomicBool = AtomicBool::new(false);

pub async fn run() -> Exit {
    let store = match store::open().await {
        Ok(store) => store,
        Err(e) => return Exit::Failed(e),
    };
    let bot = Bot::builder()
        .with_backend(store)
        .on_qr_code(|code, _| async move { emit(Event::Qr(code)) })
        .on_connected(|_| async { emit(Event::Connected) })
        .on_logged_out(|_| async { LOGGED_OUT.store(true, Ordering::SeqCst) })
        .on_message(|ctx| async move {
            if let Some(text) = ctx.message.text_content() {
                emit(Event::Message {
                    chat: ctx.info.source.chat.to_string(),
                    mine: ctx.info.source.is_from_me,
                    text: text.to_string(),
                });
            }
        })
        .build()
        .await;
    let bot = match bot {
        Ok(bot) => bot,
        Err(e) => return Exit::Failed(e.to_string()),
    };
    let reason = bot.run().await;
    if LOGGED_OUT.swap(false, Ordering::SeqCst) || removed(&reason) {
        Exit::LoggedOut
    } else {
        Exit::Dropped
    }
}

fn removed(reason: &RunCompletionReason) -> bool {
    matches!(
        reason,
        RunCompletionReason::AutoReconnectDisabled {
            protocol_error: Some(
                ProtocolTerminalReason::ConnectFailure(ConnectFailureReason::LoggedOut)
                    | ProtocolTerminalReason::Conflict(ConflictKind::DeviceRemoved)
            ),
            ..
        }
    )
}
