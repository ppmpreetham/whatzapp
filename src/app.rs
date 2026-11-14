use blenderui::prelude::*;
use std::path::PathBuf;
use time::Time;
use crate::components::chat::chat;

#[component]
pub fn App() {
    rsx! {
      <body class="m-0 min-h-screen flex flex-col bg-zinc-900 text-white">
          <chat icon_path={PathBuf::new()} title="Alice" content="See you tomorrow!" time={Time::from_hms(9, 41, 0).unwrap()} pinned={true} unread={true} />
          <chat icon_path={PathBuf::new()} title="Bob" content="A very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very very long message" time={Time::from_hms(18, 5, 0).unwrap()} pinned={false} unread={false} />
      </body>
    }
}
