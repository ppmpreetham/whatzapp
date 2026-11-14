use std::path::PathBuf;
use blenderui::prelude::*;
use crate::utils;

#[component]
pub fn Avatar(chat_avatar: Option<PathBuf>, chat_name: SharedString) {
    let initial = utils::initial(&chat_name);
    rsx! {
        <div class="size-12 shrink-0 rounded-full overflow-hidden flex items-center justify-center" style={utils::style(&[("background-color", "secondary.background"), ("color", "foreground")])}>
            {if let Some(path) = chat_avatar {
                <img src={path.to_string_lossy().into_owned()} class="size-full" />
            } else {
                <span class="text-lg font-medium">{initial}</span>
            }}
        </div>
    }
}
