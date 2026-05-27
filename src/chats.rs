use gpui::*;
use gpui_component::avatar::Avatar;
use gpui_component::list::{ListDelegate, ListItem, ListState};
use gpui_component::{Icon, IconName, IndexPath};
use gpui_rsx::rsx;

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

pub struct ChatsDelegate {
    pub chats: Vec<ChatUser>,
    pub selected_index: Option<IndexPath>,
}

impl ChatsDelegate {
    pub fn new() -> Self {
        Self {
            chats: get_dummy_chats(),
            selected_index: Some(IndexPath::new(0).section(0)),
        }
    }
}

impl ListDelegate for ChatsDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.chats.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let user = self.chats.get(ix.row)?;

        Some(
            ListItem::new(ix)
                .selected(Some(ix) == self.selected_index)
                .child(rsx! {
                    <div class="rounded-lg p-3 flex flex-row gap-3 items-center w-full">

                        <div class="rounded-full bg-zinc-200">
                            {Avatar::new()
                                .name(user.title.to_string())
                                .src(user.icon_path.to_string())
                                .placeholder(
                                    Icon::new(IconName::User)
                                )
                            }
                        </div>

                        <div class="flex-1 min-w-0">

                            <div class="flex flex-row justify-between items-center">
                                <div class="font-semibold truncate">
                                    {user.title.clone()}
                                </div>

                                <div class="text-xs text-zinc-500 whitespace-nowrap">
                                    {user.time.clone()}
                                </div>
                            </div>

                            <div class="flex flex-row justify-between items-center gap-2">

                                <div class="text-sm text-zinc-500 truncate">
                                    {user.content.clone()}
                                </div>

                                {if user.unread_msgs > 0 {
                                    rsx! {
                                        <div class="flex items-center justify-center size-20 bg-green-500 rounded-full text-xs text-white">
                                            {format!("{}", user.unread_msgs)}
                                        </div>
                                    }
                                } else {
                                    rsx! { <div /> }
                                }}

                            </div>

                            {if user.is_pinned {
                                rsx! {
                                    <div class="text-xs text-yellow-500 mt-1">
                                        {"Pinned"}
                                    </div>
                                }
                            } else {
                                rsx! { <div /> }
                            }}

                        </div>

                    </div>
                })
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.delegate_mut()
                        .set_selected_index(Some(ix), window, cx);
                }))
        )
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }
}
