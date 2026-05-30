use crate::AppState;
use gpui::prelude::*;
use gpui::*;
use gpui_component::avatar::Avatar;
use gpui_component::list::{ListDelegate, ListItem, ListState};
use gpui_component::menu::ContextMenuExt;
use gpui_component::{Icon, IconName, IndexPath};
use gpui_rsx::rsx;
use nucleo::{Config, Nucleo, Utf32Str};

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

pub struct ChatsDelegate {
    pub state: Entity<AppState>,
    pub filtered_indices: Vec<usize>,
    pub selected_index: Option<IndexPath>,
}

impl ChatsDelegate {
    pub fn new(state: Entity<AppState>, cx: &App) -> Self {
        let len = state.read(cx).chats.len();
        Self {
            state,
            filtered_indices: (0..len).collect(),
            selected_index: Some(IndexPath::new(0).section(0)),
        }
    }
}

impl ListDelegate for ChatsDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.filtered_indices.len()
    }

    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Task<()> {
        use nucleo::{
            Config, Matcher, Utf32Str,
            pattern::{CaseMatching, Normalization, Pattern},
        };

        let len = self.state.read(cx).chats.len();

        if query.is_empty() {
            self.filtered_indices = (0..len).collect();
            return Task::ready(());
        }

        let mut matcher = Matcher::new(Config::DEFAULT);
        let pattern = Pattern::parse(query, CaseMatching::Smart, Normalization::Smart);

        let chats = self.state.read(cx).chats.clone();

        let mut scored: Vec<(u32, usize)> = chats
            .iter()
            .enumerate()
            .filter_map(|(idx, chat)| {
                let searchable = format!("{} {}", chat.title, chat.content);
                let mut buf = Vec::new();
                let utf32 = Utf32Str::new(&searchable, &mut buf);

                pattern.score(utf32, &mut matcher).map(|score| (score, idx))
            })
            .collect();

        scored.sort_by(|a, b| b.0.cmp(&a.0));
        self.filtered_indices = scored.into_iter().map(|(_, idx)| idx).collect();
        Task::ready(())
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let actual_idx = *self.filtered_indices.get(ix.row)?;
        let user = self.state.read(cx).chats.get(actual_idx)?.clone();

        Some(
            ListItem::new(ix)
                .selected(Some(ix) == self.selected_index)
                .child(rsx! {
                    <div class="rounded-lg p-3 flex flex-row gap-3 items-center w-full">
                        <div class="rounded-full bg-zinc-200">
                            {Avatar::new()
                                .name(user.title.to_string())
                                .src(user.icon_path.to_string())
                                .placeholder(Icon::new(IconName::User))
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
                        .menu_with_icon("Lock chat", IconName::Check, Box::new(LockChat))
                        .menu_with_icon("Mute notifications", IconName::Bell, Box::new(MuteNotifications))
                        .menu_with_icon("Unpin chat", IconName::Minus, Box::new(UnpinChat))
                        .menu_with_icon("Mark as unread", IconName::CircleCheck, Box::new(MarkAsUnread))
                        .menu_with_icon("Add to favourites", IconName::Star, Box::new(AddToFavourites))
                        .menu_with_icon("Add to list", IconName::Plus, Box::new(AddToList))
                        .separator()
                        .menu_with_icon("Clear chat", IconName::Delete, Box::new(ClearChat))
                        .menu_with_icon("Exit group", IconName::ArrowLeft, Box::new(ExitGroup))
                })).on_click(cx.listener(move |this, _, window, cx| {
                    this.delegate_mut().set_selected_index(Some(ix), window, cx);

                    let state = this.delegate().state.clone();
                    state.update(cx, |state, _| {
                        state.selected_chat = actual_idx;
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
