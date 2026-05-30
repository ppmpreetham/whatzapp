use std::rc::Rc;

use gpui::*;
use gpui_component::{
    Icon, IconName, VirtualListScrollHandle,
    button::Button,
    input::{Input, InputState},
    v_virtual_list,
};
use gpui_rsx::rsx;

use crate::{AppState, Message};

pub struct ChatView {
    state: Entity<AppState>,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    scroll_handle: VirtualListScrollHandle,
    input: Entity<InputState>,
}

impl ChatView {
    pub fn new(state: Entity<AppState>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            InputState::new(window, cx)
                .auto_grow(1, 5)
                .placeholder("Type a message...")
        });

        let mut this = Self {
            state,
            item_sizes: Rc::new(vec![]),
            scroll_handle: VirtualListScrollHandle::new(),
            input,
        };

        let initial_selected = this.state.read(cx).selected_chat;
        this.update_item_sizes(initial_selected, cx);

        this
    }

    pub fn send_message(&mut self, text: String, cx: &mut Context<Self>) {
        self.state.update(cx, |state, _| {
            let selected = state.selected_chat;

            state.chats[selected].messages.push(Message {
                text: text.clone().into(),
                sender_id: 1,
            });

            state.chats[selected].content = text.into();
        });

        let selected = self.state.read(cx).selected_chat;
        self.update_item_sizes(selected, cx);
        self.scroll_handle.scroll_to_bottom();

        cx.notify();
    }

    fn update_item_sizes(&mut self, selected_chat: usize, cx: &App) {
        self.item_sizes = Rc::new(
            self.state.read(cx).chats[selected_chat]
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
        let selected = self.state.read(cx).selected_chat;
        self.update_item_sizes(selected, cx);

        let current_chat = {
            let state = self.state.read(cx);
            state.chats[state.selected_chat].clone()
        };

        rsx! {
            <div class="flex flex-col size-full bg-zinc-900">
                <div class="flex flex-row justify-between items-center p-3 border-b border-zinc-800">
                    <div class="flex flex-row items-center gap-3">
                        <div class="size-10 rounded-full bg-zinc-700" />

                        <div class="flex flex-col">
                            <div class="font-semibold">
                                {current_chat.title.clone()}
                            </div>
                            <div class="text-xs text-zinc-500">
                                {"online"}
                            </div>
                        </div>
                    </div>

                    <div class="flex flex-row items-center gap-3">
                        {Button::new("video-call").icon(Icon::new(IconName::Search))}
                        {Button::new("voice-call").icon(Icon::new(IconName::Search))}
                        {Button::new("search").icon(Icon::new(IconName::Search))}
                        {Button::new("menu").icon(Icon::new(IconName::EllipsisVertical))}
                    </div>
                </div>

                <div class="flex-1">
                    {
                        v_virtual_list(
                            cx.entity().clone(),
                            "chat-messages",
                            self.item_sizes.clone(),
                            |view, visible_range, _window, cx| {
                                visible_range
                                    .map(|ix| {
                                        let selected = view.state.read(cx).selected_chat;
                                        let msg = &view.state.read(cx).chats[selected].messages[ix];
                                        let mine = msg.sender_id == 1;

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
                                                                .child(msg.text.clone())
                                                        )
                                                } else {
                                                    div()
                                                        .flex().flex_row()
                                                        .justify_start()
                                                        .child(
                                                            div()
                                                                .bg(rgb(0x202c33))
                                                                .rounded_lg()
                                                                .px_3()
                                                                .py_2()
                                                                .child(msg.text.clone())
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
                            .icon(Icon::new(IconName::ArrowRight))
                            .on_click(cx.listener(|this, _, window, cx| {
                                let text = this.input.read(cx).value().to_string();
                                if !text.trim().is_empty() {
                                    this.send_message(text, cx);
                                    this.input.update(cx, |input, cx| {
                                        input.replace("", window, cx);
                                    });
                                }
                            }))
                        }
                    </div>
                </div>
            </div>
        }
    }
}
