use gpui::*;
use gpui_component::avatar::Avatar;
use gpui_component::input::{Input, InputState};
use gpui_component::list::ListItem;
use gpui_component::resizable::{h_resizable, resizable_panel};
use gpui_component::{Icon, IconName};
use gpui_component::{button::Button, *};
use gpui_rsx::rsx;

mod chats;
mod theme;

struct Whatzapp {
    peoplesearch: Entity<InputState>,
}

impl Whatzapp {
    fn single_chat(
        &self,
        ix: IndexPath,
        user: &chats::ChatUser,
        is_selected: bool,
    ) -> impl IntoElement {
        ListItem::new(ix).selected(is_selected).child(rsx! {
            <div class="rounded-lg shadow-md p-3 flex flex-row gap-3 items-center w-full">
                <div class="w-auto h-auto rounded-full bg-zinc-200">
                    {Avatar::new()
                        .name(format!("{}", user.title))
                        .src(format!("{}", user.icon_path))
                        .placeholder(Icon::new(IconName::User))
                    }
                </div>

                <div class="flex-1 min-w-0">
                    <div class="flex flex-row justify-between items-center">
                        <div class="font-semibold truncate">{user.title.clone()}</div>
                        <div class="text-xs text-zinc-500 whitespace-nowrap">{user.time.clone()}</div>
                    </div>

                    <div class="flex flex-row justify-between items-center gap-2">
                        <div class="text-sm text-zinc-500 truncate">{user.content.clone()}</div>

                        {if user.unread_msgs > 0 {
                            rsx! {
                                <div class="flex items-center justify-center size-20 bg-green-500 rounded-full text-sm text-white">
                                     {format!("{}", user.unread_msgs)}
                                 </div>
                            }
                        } else {
                            rsx! { <div /> }
                        }}
                    </div>

                    {if user.is_pinned {
                        rsx! { <div class="text-xs text-yellow-600">{"Pinned"}</div> }
                    } else {
                        rsx! { <div /> }
                    }}
                </div>
            </div>
        })
    }
}

impl Render for Whatzapp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dummy_chats = chats::get_dummy_chats();
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
                                    rsx! {<div class="flex flex-col px-2 w-full mx-2">
                                        <div class="w-full flex flex-row justify-between">
                                            <h1>{"Chats"}</h1>
                                            <div class="w-full flex flex-row">
                                                {Button::new("new-chat")
                                                    .icon(Icon::new(IconName::Heart))
                                                    .tooltip(format!("New Chat"))}
                                            </div>
                                        </div>
                                        {Input::new(&self.peoplesearch).prefix(Icon::new(IconName::Search).small()).appearance(false).w_full().rounded_full().bg(rgb(0x2e2f2f)).border_1().border_color(rgb(0x2e2f2f)) }
                                        <div class="flex flex-col gap-2 w-full mt-2">
                                            {{
                                                div()
                                                    .children(
                                                        dummy_chats
                                                            .iter()
                                                            .enumerate()
                                                            .map(|(i, user)| {
                                                                let is_selected = i == 0;

                                                                self.single_chat(
                                                                    IndexPath::new(i).section(0),
                                                                    user,
                                                                    is_selected,
                                                                )
                                                            }),
                                                    )
                                            }}
                                        </div>
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
