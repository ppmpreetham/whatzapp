use blenderui::prelude::*;
use std::path::PathBuf;
use time::Time;
use crate::components::chat::ChatProps;
use crate::theme;
use crate::components::personchat::PersonChat;

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
pub fn App() {
    let chats = sample_chats();
    let selected = use_signal(0usize);
    let current = chats[selected.get()].title.clone();

    rsx! {
      <body class="m-0 size-full flex flex-row" style={format!("background-color:{};color:{}", theme::color("background"), theme::color("foreground"))}>
        <div class="w-96 shrink-0 flex flex-col overflow-hidden border-r" style={format!("border-color:{}", theme::color("border"))}>
          {for (i, c) in chats.into_iter().enumerate() {
            <div onClick={move || selected.set(i)}>
              <chat {...c} />
            </div>
          }}
        </div>
        <div class="flex-1 min-w-0">
          <PersonChat name={current} />
        </div>
      </body>
    }
}
