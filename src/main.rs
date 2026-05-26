use gpui::*;
use gpui_component::input::{Input, InputState};
use gpui_component::resizable::{h_resizable, resizable_panel};
use gpui_component::{button::Button, *};
use gpui_rsx::rsx;
mod theme;
use gpui_component::{Icon, IconName};

struct Whatzapp {
    peoplesearch: Entity<InputState>,
}

impl Render for Whatzapp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        rsx! {
            <div class="flex flex-col gap-4 p-4 bg-white size-full text-white bg-black h-full">
                <div class="flex flex-row gap-4 items-center h-full">
                    // options
                    <div class="flex flex-col p-2">

                    </div>
                    {h_resizable("whatzapp-main")
                    // people area, all the people
                        .child(
                            resizable_panel()
                                .size(px(320.))
                                .size_range(px(240.)..px(520.))
                                .child(
                                    rsx! {<div class="flex flex-col p-2 w-full">
                                        <div class="w-full flex flex-row justify-between">
                                            <h1>{"Chats"}</h1>
                                            <div class="w-full flex flex-row">
                                                {Button::new("new-chat")
                                                    .icon(Icon::new(IconName::Heart))
                                                    .tooltip(format!("New Chat"))}
                                            </div>
                                        </div>
                                        {Input::new(&self.peoplesearch).prefix(Icon::new(IconName::Search).small()).appearance(false).w_full().rounded_full().bg(rgb(0x2e2f2f)).border_1().border_color(rgb(0x2e2f2f))}
                                    </div>}

                                )
                        )
                        // chat area here
                        .child(
                            resizable_panel()
                                .child(
                                    // chat area here
                                    div().child("chat...")
                                )
                        )}
                    // print state
                    <div class="flex flex-col border border-gray-300 p-2">
                        // {"Current text: "}
                        // {self.search.read(cx).value()}
                    </div>
                    // chat
                    <div class="flex flex-col">
                        // title bar
                        <div class="flex flex-row">

                        </div>
                    </div>
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
                Whatzapp { peoplesearch }
            });
            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();
    });
}
