use std::path::PathBuf;
use blenderui::prelude::*;
use crate::theme;

#[component]
pub fn Avatar(chat_avatar: Option<PathBuf>, chat_name: SharedString) {
    let initial = chat_name.chars().next().unwrap_or('?').to_string();
    rsx! {
        <div class="size-12 shrink-0 rounded-full overflow-hidden flex items-center justify-center" style={format!("background-color:{};color:{}", theme::color("secondary.background"), theme::color("foreground"))}>
            {if let Some(path) = chat_avatar {
                <img src={path.to_string_lossy().into_owned()} class="size-full" />
            } else {
                <span class="text-lg font-medium">{initial}</span>
            }}
        </div>
    }
}
