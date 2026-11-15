use whatsapp_rust::prelude::*;

const DB: &str = "whatsapp.db";

pub async fn open() -> Result<SqliteStore, String> {
    SqliteStore::open(DB).await.map_err(|e| e.to_string())
}

pub fn forget() {
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{DB}{suffix}"));
    }
}
