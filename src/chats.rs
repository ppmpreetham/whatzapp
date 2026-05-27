use gpui::SharedString;

#[derive(Clone)]
pub struct ChatUser {
    pub icon_path: SharedString,
    pub title: SharedString,
    pub content: SharedString,
    pub time: SharedString,
    pub is_pinned: bool,
    pub unread_msgs: usize,
}

pub fn get_dummy_chats() -> Vec<ChatUser> {
    vec![
        ChatUser {
            icon_path: "avatar1.png".into(),
            title: "Alice".into(),
            content: "Typing…".into(),
            time: "10:42".into(),
            is_pinned: true,
            unread_msgs: 3,
        },
        ChatUser {
            icon_path: "avatar2.png".into(),
            title: "Bob".into(),
            content: "See you tomorrow!".into(),
            time: "09:15".into(),
            is_pinned: false,
            unread_msgs: 0,
        },
        ChatUser {
            icon_path: "avatar3.png".into(),
            title: "Rust Community".into(),
            content: "New update released!".into(),
            time: "Yesterday".into(),
            is_pinned: true,
            unread_msgs: 12,
        },
    ]
}
