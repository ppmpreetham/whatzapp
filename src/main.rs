use gpui::*;
use gpui::{Action, actions};
use gpui_component::list::{List, ListState};
use gpui_component::resizable::{h_resizable, resizable_panel};
use gpui_component::{button::Button, *};
use gpui_component_assets::Assets;
use gpui_rsx::rsx;

mod chat;
mod chats;
mod theme;

use chat::ChatView;
use chats::ChatsDelegate;

#[derive(Clone)]
pub struct Message {
    pub text: SharedString,
    pub sender_id: u64,
}

#[derive(Clone)]
pub struct Chat {
    pub icon_path: SharedString,
    pub title: SharedString,
    pub content: SharedString,
    pub time: SharedString,
    pub is_pinned: bool,
    pub unread_msgs: usize,
    pub messages: Vec<Message>,
}

pub struct AppState {
    pub selected_chat: usize,
    pub chats: Vec<Chat>,
}

struct Whatzapp {
    chats: Entity<ListState<ChatsDelegate>>,
    chat_view: Entity<ChatView>,
    state: Entity<AppState>,
}

actions!(custom_menu, [NewGrp, StarMsgs, MrkAllRead, LogOut, AppLock]);

impl Render for Whatzapp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        rsx! {
            <div class="flex flex-col gap-4 p-4 bg-black text-white size-full h-full">
                <div class="flex flex-row gap-4 h-full">
                    {h_resizable("whatzapp-main")
                        .child(
                            resizable_panel()
                                .size(px(320.))
                                .size_range(px(240.)..px(520.))
                                .child(
                                    rsx! {
                                        <div class="flex flex-col px-2 w-full h-full gap-3">
                                            <div class="flex flex-row justify-between items-center">
                                                <h1 class="text-xl font-bold">
                                                    {"Chats"}
                                                </h1>
                                                {Button::new("new-chat")
                                                    .icon(Icon::new(IconName::Plus))
                                                    .tooltip("New Chat")
                                                }
                                            </div>
                                            <div class="flex-1 overflow-hidden">
                                                {List::new(&self.chats)}
                                            </div>
                                        </div>
                                    }
                                )
                        )
                        .child(
                            resizable_panel()
                                .child(self.chat_view.clone())
                        )
                    }
                </div>
            </div>
        }
    }
}

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        gpui_component::init(cx);
        theme::init(cx);

        cx.open_window(WindowOptions::default(), |window, cx| {
            let view = cx.new(|cx| {
                // TODO: use B+tree FTS5 for actual searching here
                let state = cx.new(|_| AppState {
                    selected_chat: 0,
                    chats: vec![
                        Chat {
                            icon_path: "avatar1.png".into(),
                            title: "Alice".into(),
                            content: "Typing…".into(),
                            time: "10:42".into(),
                            is_pinned: true,
                            unread_msgs: 3,
                            messages: vec![
                                Message {
                                    text: "Hey".into(),
                                    sender_id: 2,
                                },
                                Message {
                                    text: "Hello!".into(),
                                    sender_id: 1,
                                },
                            ],
                        },
                        Chat {
                            icon_path: "avatar2.png".into(),
                            title: "Bob".into(),
                            content: "See you tomorrow!".into(),
                            time: "09:15".into(),
                            is_pinned: false,
                            unread_msgs: 0,
                            messages: vec![
                                Message {
                                    text: "Rust is awesome".into(),
                                    sender_id: 2,
                                },
                                Message {
                                    text: "Absolutely".into(),
                                    sender_id: 1,
                                },
                            ],
                        },
                        Chat {
                            icon_path: "avatar3.png".into(),
                            title: "Rust Community".into(),
                            content: "New update releaseddddddddddddddddddddddddddddddddddd!"
                                .into(),
                            time: "Yesterday".into(),
                            is_pinned: true,
                            unread_msgs: 12,
                            messages: vec![Message {
                                text: "New update releaseddddddddddddddddddddddddddddddddddd!"
                                    .into(),
                                sender_id: 2,
                            }],
                        },
                    ],
                });

                let chat_view = cx.new(|cx| ChatView::new(state.clone(), window, cx));

                let chats = cx.new(|cx| {
                    ListState::new(ChatsDelegate::new(state.clone(), cx), window, cx)
                        .searchable(true)
                });

                Whatzapp {
                    chats,
                    chat_view,
                    state,
                }
            });
            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();
    });
}
