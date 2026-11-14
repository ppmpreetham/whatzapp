use blenderui::prelude::*;
use std::path::PathBuf;
use time::Time;
use crate::components::chat::chat;
use crate::components::personchat::PersonChat;

#[component]
pub fn App() {
    rsx! {
      <body class="m-0 size-full flex flex-row bg-zinc-900 text-white">
        <div class="w-96 shrink-0 flex flex-col overflow-hidden border-r border-white/10">
            <chat icon_path={PathBuf::new()} title="Alice" content="See you tomorrow!" time={Time::from_hms(9, 41, 0).unwrap()} pinned={true} unread={true} />
            <chat icon_path={PathBuf::new()} title="Bob" content="A very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very long message" time={Time::from_hms(18, 5, 0).unwrap()} pinned={false} unread={false} />
        </div>
        <div class="flex-1 min-w-0 flex flex-col">
          <PersonChat />
        </div>
      </body>
    }
}
