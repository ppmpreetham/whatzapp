use blenderui::prelude::*;
use std::path::PathBuf;
use time::Time;
use crate::components::chat::ChatProps;
use crate::{theme, utils};
use crate::components::conversation::Conversation;
use crate::components::personchat::{messages::MessageList, PersonChat};

fn sample_chats() -> Vec<ChatProps> {
    let chat = |title: &'static str, content: &'static str, (h, m): (u8, u8), pinned, unread| ChatProps {
        icon_path: PathBuf::new(),
        title: title.into(),
        content: content.into(),
        time: Time::from_hms(h, m, 0).unwrap(),
        pinned,
        unread,
    };
    vec![
        chat("Alice", "See you tomorrow!", (9, 41), true, true),
        chat("Bob", "A very very very very very very very very very very very very very very very very long message", (18, 5), false, false),
        chat("Carol", "Thanks, that works for me", (14, 22), false, true),
    ]
}

#[component]
pub fn App(lists: Vec<MessageList>) {
    let chats = sample_chats();
    let selected = use_signal(0usize);
    let current = chats[selected.get()].title.clone();
    let count = chats.len();
    let cycle = move |e: &KeyDownEvent| {
        let m = e.keystroke.modifiers;
        if m.control && e.keystroke.key.to_ascii_lowercase().ends_with("tab") {
            selected.set((selected.get() + if m.shift { count - 1 } else { 1 }) % count);
        }
    };

    rsx! {
      <body class="m-0 size-full flex flex-row" onKeyDown={move |e| cycle(e)} style={utils::style(&[("background-color", "background"), ("color", "foreground")])}>
        <div class="w-96 shrink-0 flex flex-col overflow-hidden border-r" style={utils::style(&[("border-color", "border")])}>
          <div class="flex items-center justify-between px-4 py-3">
            <span class="text-lg font-semibold">"Chats"</span>
            <div class={format!("size-8 rounded-full flex items-center justify-center cursor-pointer hover:bg-[{}]", theme::color("secondary.active.background"))}>
              <img src="public/icons/search.svg" width="16" height="16" />
            </div>
          </div>
          {for (i, c) in chats.into_iter().enumerate() {
            <div class={if selected.get() == i { format!("bg-[{}]", theme::color("list.active.background")) } else { String::new() }} onClick={move || selected.set(i)}>
              <chat {...c} />
            </div>
          }}
        </div>
        <div class="flex-1 min-w-0">
          {if std::env::var_os("WZ_PERSONCHAT").is_none() {
            rsx! { <Conversation name={current} chat={selected.get()} /> }
          } else {
            rsx! { <PersonChat name={current} list={lists[selected.get()].clone()} /> }
          }}
        </div>
      </body>
    }
}
