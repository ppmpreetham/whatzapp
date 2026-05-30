use crate::AppState;
use crate::chat::ChatView;
use gpui::prelude::*;
use gpui::*;
use gpui_component::avatar::Avatar;
use gpui_component::list::{ListDelegate, ListItem, ListState};
use gpui_component::menu::ContextMenuExt;
use gpui_component::{Icon, IconName, IndexPath};
use gpui_rsx::rsx;
use nucleo::{Config, Nucleo, Utf32Str};

// TODO: add functionality to these
actions!(
    whatsapp_chats,
    [
        ArchiveChat,
        LockChat,
        MuteNotifications,
        UnpinChat,
        MarkAsUnread,
        AddToFavourites,
        AddToList,
        ClearChat,
        ExitGroup
    ]
);

#[derive(Clone)]
pub struct ChatUser {
    pub icon_path: SharedString,
    pub title: SharedString,
    pub content: SharedString,
    pub time: SharedString,
    pub is_pinned: bool,
    pub unread_msgs: usize,
    // TODO: is_typing state
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
            content: "New update releaseddddddddddddddddddddddddddddddddddd!".into(),
            time: "Yesterday".into(),
            is_pinned: true,
            unread_msgs: 12,
        },
    ]
}

pub struct ChatsDelegate {
    state: Entity<AppState>,
    pub all_chats: Vec<ChatUser>,
    pub filtered_chats: Vec<ChatUser>,
    pub selected_index: Option<IndexPath>,
}

impl ChatsDelegate {
    pub fn new(state: Entity<AppState>) -> Self {
        let chats = get_dummy_chats();

        Self {
            state,

            all_chats: chats.clone(),
            filtered_chats: chats,
            selected_index: Some(IndexPath::new(0).section(0)),
        }
    }
}

impl ListDelegate for ChatsDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.filtered_chats.len()
    }

    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> Task<()> {
        use nucleo::{
            Config, Matcher, Utf32Str,
            pattern::{CaseMatching, Normalization, Pattern},
        };

        if query.is_empty() {
            self.filtered_chats = self.all_chats.clone();
            return Task::ready(());
        }

        let mut matcher = Matcher::new(Config::DEFAULT);
        let pattern = Pattern::parse(query, CaseMatching::Smart, Normalization::Smart);
        let mut scored: Vec<(u32, ChatUser)> = self
            .all_chats
            .iter()
            .filter_map(|chat| {
                let searchable = format!("{} {}", chat.title, chat.content);

                let mut buf = Vec::new();
                let utf32 = Utf32Str::new(&searchable, &mut buf);

                pattern
                    .score(utf32, &mut matcher)
                    .map(|score| (score, chat.clone()))
            })
            .collect();

        scored.sort_by(|a, b| b.0.cmp(&a.0));
        self.filtered_chats = scored.into_iter().map(|(_, chat)| chat).collect();
        Task::ready(())
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let user = self.filtered_chats.get(ix.row)?;

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

                                <div class="flex flex-row gap-2">
                                {if user.unread_msgs > 0 {
                                    rsx! {
                                        <div class="flex items-center justify-center size-20 bg-green-500 rounded-full text-xs text-white">
                                            {format!("{}", user.unread_msgs)}
                                        </div>
                                    }.into_any_element()
                                } else {
                                    div().size_0().into_any_element()
                                }}
                                // change this to the actual icon
                                {if user.is_pinned {
                                    rsx! {
                                        <div class="text-xs text-yellow-500 mt-1">
                                            {"Pinned"}
                                        </div>
                                    }.into_any_element()
                                } else {
                                    div().size_0().into_any_element()
                                }}
                                </div>
                            </div>
                        </div>
                    </div>
                }.context_menu(|menu, _window, _cx| {
                    menu.menu_with_icon("Archive chat", IconName::Inbox, Box::new(ArchiveChat))
                        .menu_with_icon("Lock chat", IconName::Check, Box::new(LockChat)) // No Lock icon; using CircleCheck/Check alternative
                        .menu_with_icon("Mute notifications", IconName::Bell, Box::new(MuteNotifications)) // Standard Bell
                        .menu_with_icon("Unpin chat", IconName::Minus, Box::new(UnpinChat)) // Generic removal indicator
                        .menu_with_icon("Mark as unread", IconName::CircleCheck, Box::new(MarkAsUnread))
                        .menu_with_icon("Add to favourites", IconName::Star, Box::new(AddToFavourites))
                        .menu_with_icon("Add to list", IconName::Plus, Box::new(AddToList))
                        .separator()
                        .menu_with_icon("Clear chat", IconName::Delete, Box::new(ClearChat))
                        .menu_with_icon("Exit group", IconName::ArrowLeft, Box::new(ExitGroup))
                })).on_click(cx.listener(move |this, _, window, cx| {
                    this.delegate_mut()
                        .set_selected_index(Some(ix), window, cx);
                    let state =
                        this.delegate().state.clone();

                    state.update(cx, |state, _| {
                        state.selected_chat = ix.row;
                    });
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
