use std::path::PathBuf;
use crate::components::avatar::Avatar;

use blenderui::prelude::*;

#[component]
pub fn PersonChatHeader(chatName: SharedString, chatAvatar: Option<PathBuf>) {
    rsx! {
      <div className="flex items-center p-3 bg-light-secondary dark:bg-received-bubble-dark-bg border-b border-gray-300 dark:border-gray-700">
        // {onBack && (
        //   <button onClick={onBack} className="mr-4 md:hidden">
        //     <GoBackIcon />
        //   </button>
        // )}
        <div className="flex items-center gap-3">
          <Avatar chat_avatar={chatAvatar} chat_name={chatName.clone()} />
          <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-100">{chatName}</h2>
        </div>
      </div>

    }
}
