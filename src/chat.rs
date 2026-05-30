use std::rc::Rc;

use gpui::*;
use gpui_component::{
    Icon, IconName, VirtualListScrollHandle,
    button::Button,
    input::{Input, InputState},
    v_virtual_list,
};
use gpui_rsx::rsx;

use crate::AppState;

pub struct ChatView {
    conversations: Vec<Conversation>,
    state: Entity<AppState>,

    item_sizes: Rc<Vec<Size<Pixels>>>,
    scroll_handle: VirtualListScrollHandle,
    input: Entity<InputState>,
}

pub struct Conversation {
    pub title: SharedString,
    pub messages: Vec<Message>,
}

pub struct Message {
    pub text: SharedString,
    pub sender_id: u64,
}

impl ChatView {
    pub fn new(state: Entity<AppState>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            InputState::new(window, cx)
                .auto_grow(1, 5)
                .placeholder("Type a message...")
        });

        let conversations = vec![
            Conversation {
                title: "Alice".into(),
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
            Conversation {
                title: "Bob".into(),
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
        ];

        let mut this = Self {
            conversations,
            state,

            item_sizes: Rc::new(vec![]),
            scroll_handle: VirtualListScrollHandle::new(),
            input,
        };

        this.update_item_sizes(0);

        this
    }

    pub fn send_message(&mut self, text: String, cx: &mut Context<Self>) {
        self.conversations[self.state.read(cx).selected_chat]
            .messages
            .push(Message {
                text: text.into(),
                sender_id: 1,
            });

        let selected = self.state.read(cx).selected_chat;
        self.update_item_sizes(selected);

        self.scroll_handle.scroll_to_bottom();

        cx.notify();
    }

    fn update_item_sizes(&mut self, selected_chat: usize) {
        self.item_sizes = Rc::new(
            self.conversations[selected_chat]
                .messages
                .iter()
                .map(|msg| {
                    let lines = (msg.text.len() / 40).max(1);

                    size(px(350.), px(40. + lines as f32 * 18.))
                })
                .collect(),
        );
    }
}

impl Render for ChatView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        rsx! {
            <div class="flex flex-col size-full bg-zinc-900">

                <div class="flex flex-row justify-between items-center p-3 border-b border-zinc-800">
                    <div class="flex flex-row items-center gap-3">
                        <div class="size-10 rounded-full bg-zinc-700" />

                        <div class="flex flex-col">
                            <div class="font-semibold">
                                {self.conversations[self.state.read(cx).selected_chat]
                                    .title
                                    .clone()}
                            </div>

                            <div class="text-xs text-zinc-500">
                                {"online"}
                            </div>
                        </div>
                    </div>

                    <div class="flex flex-row items-center gap-3">
                        {Button::new("video-call")
                            .icon(Icon::new(IconName::Search))}

                        {Button::new("voice-call")
                            .icon(Icon::new(IconName::Search))}

                        {Button::new("search")
                            .icon(Icon::new(IconName::Search))}

                        {Button::new("menu")
                            .icon(Icon::new(IconName::EllipsisVertical))}
                    </div>
                </div>

                <div class="flex-1">
                    {
                        v_virtual_list(
                            cx.entity().clone(),
                            "chat-messages",
                            self.item_sizes.clone(),
                            |view, visible_range, _, _| {
                                visible_range
                                    .map(|ix| {
                                        let selected = 0;

                                        let msg =
                                            &view.conversations[selected]
                                                .messages[ix];

                                        let mine =
                                            msg.sender_id == 1;

                                        div()
                                            .w_full()
                                            .h(view.item_sizes[ix].height)
                                            .px_4()
                                            .py_2()
                                            .child(
                                                if mine {
                                                    div()
                                                        .flex().flex_row()
                                                        .justify_end()
                                                        .child(
                                                            div()
                                                                .bg(rgb(0x005c4b))
                                                                .rounded_lg()
                                                                .px_3()
                                                                .py_2()
                                                                .child(
                                                                    msg.text.clone()
                                                                )
                                                        )
                                                } else {
                                                    div().flex().flex_row()
                                                        .justify_start()
                                                        .child(
                                                            div()
                                                                .bg(rgb(0x202c33))
                                                                .rounded_lg()
                                                                .px_3()
                                                                .py_2()
                                                                .child(
                                                                    msg.text.clone()
                                                                )
                                                        )
                                                }
                                            )
                                    })
                                    .collect()
                            },
                        )
                        .track_scroll(&self.scroll_handle)
                    }
                </div>

                <div class="p-3 border-t border-zinc-800">
                    <div class="flex flex-row gap-2 items-end">

                        {Input::new(&self.input)}

                        {Button::new("send")
                            .icon(
                                Icon::new(
                                    IconName::ArrowRight
                                )
                            )}
                    </div>
                </div>
            </div>
        }
    }
}
