use blenderui::prelude::*;

pub mod header;
pub mod singlemessage;
pub mod messages;

use crate::components::input::MessageInput;
use header::PersonChatHeader;
use messages::{MessageList, Messages};

#[component]
pub fn PersonChat(name: SharedString, list: MessageList) {
    rsx! {
      <div class="flex flex-col size-full">
        <PersonChatHeader chatName={name} />
        <Messages list={list} />
        <MessageInput />
      </div>
    }
}
