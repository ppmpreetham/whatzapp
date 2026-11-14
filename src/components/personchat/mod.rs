use blenderui::prelude::*;

pub mod header;

use header::PersonChatHeader;

#[component]
pub fn PersonChat(name: SharedString) {
    rsx! {
      <div class="flex flex-col">
        <PersonChatHeader chatName={name} />
      </div>
    }
}
