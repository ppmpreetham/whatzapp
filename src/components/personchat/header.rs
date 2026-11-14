use std::path::PathBuf;
use crate::components::avatar::Avatar;

use blenderui::prelude::*;
use crate::theme;

#[component]
pub fn PersonChatHeader(chatName: SharedString, chatAvatar: Option<PathBuf>) {
    rsx! {
      <div className="flex items-center p-3 border-b" style={format!("background-color:{};border-color:{};color:{}", theme::color("secondary.background"), theme::color("border"), theme::color("foreground"))}>
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
