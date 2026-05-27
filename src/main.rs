use gpui::*;
use gpui_component::input::{Input, InputState};
use gpui_component::list::{List, ListState};
use gpui_component::resizable::{h_resizable, resizable_panel};
use gpui_component::{button::Button, *};
use gpui_rsx::rsx;

mod chats;
mod theme;

use chats::ChatsDelegate;

struct Whatzapp {
    peoplesearch: Entity<InputState>,
    chats: Entity<ListState<ChatsDelegate>>,
}

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
                                            {Input::new(&self.peoplesearch)
                                                .prefix(
                                                    Icon::new(IconName::Search)
                                                        .small()
                                                )
                                                .appearance(false)
                                                .w_full()
                                                .rounded_full()
                                                .bg(rgb(0x2e2f2f))
                                                .border_1()
                                                .border_color(rgb(0x2e2f2f))
                                            }
                                            <div class="flex-1 overflow-hidden">
                                                {List::new(&self.chats)}
                                            </div>
                                        </div>
                                    }
                                )
                        )
                        .child(
                            resizable_panel()
                                .child(
                                    rsx! {
                                        <div class="flex items-center justify-center size-full">
                                            {"chat..."}
                                        </div>
                                    }
                                )
                        )
                    }
                </div>
            </div>
        }
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        theme::init(cx);

        cx.open_window(WindowOptions::default(), |window, cx| {
            let view = cx.new(|cx| {
                let peoplesearch =
                    cx.new(|cx| InputState::new(window, cx).placeholder("Search..."));
                let chats = cx.new(|cx| ListState::new(ChatsDelegate::new(), window, cx));
                Whatzapp {
                    peoplesearch,
                    chats,
                }
            });
            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();
    });
}
