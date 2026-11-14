use std::path::PathBuf;
use blenderui::prelude::*;

#[component]
pub fn Avatar(chat_avatar: Option<PathBuf>, chat_name: SharedString) {
    let initial = chat_name.chars().next().unwrap_or('?').to_string();
    rsx! {
        <div class="size-12 shrink-0 rounded-full bg-zinc-600 overflow-hidden flex items-center justify-center">
            {if let Some(path) = chat_avatar {
                <img src={path.to_string_lossy().into_owned()} class="size-full" />
            } else {
                <span class="text-lg font-medium text-white">{initial}</span>
            }}
        </div>
    }
}
