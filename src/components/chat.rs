use std::path::PathBuf;
use time::Time;
use blenderui::prelude::*;

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
    let initial = String::from(title
        .chars()
        .next()
        .and_then(|c| c.to_uppercase().next())
        .unwrap_or('?'));

    rsx! {
        <div class="flex items-center p-3 cursor-pointer border-b border-white/10 hover:bg-zinc-800">
            <div class="size-12 mr-4 shrink-0 rounded-full bg-zinc-600 overflow-hidden flex items-center justify-center">
                <span class="text-lg font-medium text-white">{initial}</span>
            </div>
            <div class="flex-1 min-w-0">
                <div class="flex justify-between items-center mb-1">
                    <div class="flex-1 min-w-0 font-medium text-white truncate">{title}</div>
                    <span class="text-xs text-zinc-400">{stamp}</span>
                </div>
                <div class="flex justify-between items-center gap-2">
                    <div class="flex-1 min-w-0 h-5 text-sm text-zinc-400 truncate">{content}</div>
                    <div class="flex items-center gap-2 shrink-0">
                        {pinned.then(|| rsx! { <div class="text-xs text-zinc-400">"pinned"</div> })}
                        {unread.then(|| rsx! { <div class="w-2 h-2 rounded-full bg-emerald-500" /> })}
                    </div>
                </div>
            </div>
        </div>
    }
}
