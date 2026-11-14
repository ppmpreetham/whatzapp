// for first qr, bro needs to login thru his phone

use tokio::sync::{watch, watch::Receiver};
use whatsapp_rust::prelude::*;
use anyhow::Result;

pub async fn start() -> Result<Receiver<Option<String>>> {
    let (tx, rx) = watch::channel(None);
    let store = SqliteStore::open("whatsapp.db").await?;

    let bot = Bot::builder()
        .with_backend(store)
        .on_qr_code(move |code, _| {
            let tx = tx.clone();
            async move {
                tx.send_replace(Some(code));
            }
        })
        .build()
        .await?;

    tokio::spawn(async move {
        bot.run().await;
    });

    Ok(rx)
}
