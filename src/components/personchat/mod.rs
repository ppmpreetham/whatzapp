use blenderui::prelude::*;

pub mod header;

use crate::components::input::MessageInput;
use header::PersonChatHeader;

#[component]
pub fn PersonChat(name: SharedString) {
    rsx! {
      <div class="flex flex-col size-full">
        <PersonChatHeader chatName={name} />
        <div class="flex-1 min-h-0" />
        <MessageInput />
      </div>
    }
}
