use std::path::PathBuf;
use time::Time;
use blenderui::prelude::*;

#[component]
pub fn chat(icon_path: PathBuf, title: String, content: String, time: Time, pinned: bool, unread: bool) {
    // _ = icon_path;
    let stamp = format!("{:02}:{:02}", time.hour(), time.minute());

    rsx! {
        <div flex items-center p-3 cursor-pointer border-b border-white/10 hover:bg-zinc-800>
            <div size-12 mr-4 shrink-0 rounded-full bg-zinc-600 overflow-hidden flex items-center justify-center>
                <span text-lg font-medium text-white>{title.clone()}</span>
            </div>
            <div flex-1 min-w-0>
                <div flex justify-between items-center mb-1>
                    <div flex-1 min-w-0 font-medium text-white truncate>{title}</div>
                    <span text-xs text-zinc-400>{stamp}</span>
                </div>
                <div flex justify-between items-center gap-2>
                    <div flex-1 min-w-0 h-5 text-sm text-zinc-400 truncate>{content.clone()}</div>
                    <div flex items-center gap-2 shrink-0>
                        {if pinned { rsx! { <div text-xs text-zinc-400>"pinned"</div> } } else { div() }}
                        {if unread { rsx! { <div w-2 h-2 rounded-full bg-emerald-500 /> } } else { div() }}
                    </div>
                </div>
            </div>
        </div>
    }
}
