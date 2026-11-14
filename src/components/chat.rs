use std::path::PathBuf;
use time::Time;
use blenderui::prelude::*;
use crate::{theme, utils};

#[derive(Props)]
pub struct ChatProps {
    pub icon_path: PathBuf,
    pub title: SharedString,
    pub content: SharedString,
    pub time: Time,
    pub pinned: bool,
    pub unread: bool,
}

#[component]
pub fn chat(ChatProps { icon_path, title, content, time, pinned, unread }: ChatProps) {
    // TODO: once storage is done, we take the pfps from it.
    _ = icon_path;
    let stamp = format!("{:02}:{:02}", time.hour(), time.minute());
    let initial = utils::initial(&title);

    rsx! {
        <div class={format!("flex items-center p-3 cursor-pointer border-b hover:bg-[{}]", theme::color("list.hover.background"))} style={utils::style(&[("border-color", "border")])}>
            <div class="size-12 mr-4 shrink-0 rounded-full overflow-hidden flex items-center justify-center" style={utils::style(&[("background-color", "secondary.background")])}>
                <span class="text-lg font-medium">{initial}</span>
            </div>
            <div class="flex-1 min-w-0">
                <div class="flex justify-between items-center mb-1">
                    <div class="flex-1 min-w-0 font-medium truncate">{title}</div>
                    <span class="text-xs" style={utils::style(&[("color", "muted.foreground")])}>{stamp}</span>
                </div>
                <div class="flex justify-between items-center gap-2">
                    <div class="flex-1 min-w-0 h-5 text-sm truncate" style={utils::style(&[("color", "muted.foreground")])}>{content}</div>
                    <div class="flex items-center gap-2 shrink-0">
                        {pinned.then(|| rsx! { <div class="text-xs" style={utils::style(&[("color", "muted.foreground")])}>"pinned"</div> })}
                        {unread.then(|| rsx! { <div class="w-2 h-2 rounded-full" style={utils::style(&[("background-color", "primary.background")])} /> })}
                    </div>
                </div>
            </div>
        </div>
    }
}
