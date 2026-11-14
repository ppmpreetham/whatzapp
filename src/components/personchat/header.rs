use std::path::PathBuf;
use crate::components::avatar::Avatar;

use blenderui::prelude::*;
use crate::utils;

#[component]
pub fn PersonChatHeader(chatName: SharedString, chatAvatar: Option<PathBuf>) {
    rsx! {
      <div className="flex items-center p-3 border-b" style={utils::style(&[("background-color", "secondary.background"), ("border-color", "border"), ("color", "foreground")])}>
        // {onBack && (
        //   <button onClick={onBack} className="mr-4 md:hidden">
        //     <GoBackIcon />
        //   </button>
        // )}
        <div className="flex items-center gap-3">
          <Avatar chat_avatar={chatAvatar} chat_name={chatName.clone()} />
          <h2 className="text-lg font-semibold">{chatName}</h2>
        </div>
      </div>

    }
}
