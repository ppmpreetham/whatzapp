use blenderui::prelude::*;

pub mod header;

use header::PersonChatHeader;

#[component]
pub fn PersonChat() {
    rsx! {
      <div class="flex flex-col">
        <PersonChatHeader chatName="John Doe" />
      </div>
    }
}
