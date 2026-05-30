use gpui::*;
use gpui::{Action, actions};
use gpui_component::list::{List, ListState};
use gpui_component::menu::ContextMenuExt;
use gpui_component::resizable::{h_resizable, resizable_panel};
use gpui_component::{button::Button, *};
use gpui_component_assets::Assets;
use gpui_rsx::rsx;
mod chat;
mod chats;
mod theme;

use chat::ChatView;

use chats::ChatsDelegate;

struct Whatzapp {
    chats: Entity<ListState<ChatsDelegate>>,
    chat_view: Entity<ChatView>,
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
                                                    .icon(
                                                        Icon::new(IconName::Plus)
                                                    )
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
                let chats =
                    cx.new(|cx| ListState::new(ChatsDelegate::new(), window, cx).searchable(true));
                let chat_view = cx.new(|cx| ChatView::new(window, cx));
                Whatzapp { chats, chat_view }
            });
            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();
    });
}
