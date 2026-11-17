use std::sync::atomic::{AtomicBool, Ordering};
use whatsapp_rust::prelude::*;
use whatsapp_rust::types::events::{ConnectFailureReason, Event as Wa};
use whatsapp_rust::{ProtocolTerminalReason, RunCompletionReason};

use tokio::sync::mpsc::UnboundedReceiver;

use super::event::{emit, Outgoing, Event};
use super::store;

pub enum Exit {
    LoggedOut,
    Dropped,
    Failed(String),
}

static LOGGED_OUT: AtomicBool = AtomicBool::new(false);

pub async fn run(outbox: &mut UnboundedReceiver<Outgoing>) -> Exit {
    let store = match store::open().await {
        Ok(store) => store,
        Err(e) => return Exit::Failed(e),
    };
    let bot = Bot::builder()
        .with_backend(store)
        .on_qr_code(|code, _| async move { emit(Event::Qr(code)) })
        .on_connected(|_| async { emit(Event::Connected) })
        .on_logged_out(|_| async { LOGGED_OUT.store(true, Ordering::SeqCst) })
        .on_event_for(&[EventKind::HistorySync], |event, _| async move { history(&event) })
        .on_message(|ctx| async move { live(&ctx) })
        .build()
        .await;
    let bot = match bot {
        Ok(bot) => bot,
        Err(e) => return Exit::Failed(e.to_string()),
    };
    let client = bot.client();
    let run = bot.run();
    tokio::pin!(run);
    let reason = loop {
        tokio::select! {
            reason = &mut run => break reason,
            Some(out) = outbox.recv() => deliver(&client, out).await,
        }
    };
    if LOGGED_OUT.swap(false, Ordering::SeqCst) || removed(&reason) {
        Exit::LoggedOut
    } else {
        Exit::Dropped
    }
}

fn live(ctx: &MessageContext) {
    if let Some(text) = ctx.message.text_content() {
        let source = &ctx.info.source;
        let name = if source.is_from_me || source.is_group { "" } else { &ctx.info.push_name };
        emit(Event::Message { chat: source.chat.to_string(), name: name.into(), mine: source.is_from_me, text: text.to_string() });
    }
}

fn history(event: &Wa) {
    let Wa::HistorySync(sync) = event else { return };
    for chat in sync.get().into_iter().flat_map(|h| &h.conversations) {
        let name = chat.name.as_deref().or(chat.display_name.as_deref()).unwrap_or_default();
        chat.messages
            .iter()
            .filter_map(|m| m.message.as_option())
            .filter_map(|info| {
                let text = info.message.as_option()?.text_content()?;
                let mine = info.key.as_option().and_then(|k| k.from_me).unwrap_or(false);
                Some(Event::Message { chat: chat.id.clone(), name: name.into(), mine, text: text.to_string() })
            })
            .for_each(emit);
    }
}

async fn deliver(client: &Client, out: Outgoing) {
    let Ok(to) = out.chat.parse::<Jid>() else { return };
    let message = wa::Message::text(out.text);
    client.send_message(to, message).await.ok();
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
