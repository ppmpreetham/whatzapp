use gpui::*;
use gpui_component::{
    button::Button,
    input::{Input, InputState},
    *,
};
use gpui_rsx::rsx;

use gpui::{Pixels, Size, px, size};
use gpui_component::{VirtualListScrollHandle, v_virtual_list};
use std::rc::Rc;

pub struct ChatView {
    messages: Vec<Message>,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    scroll_handle: VirtualListScrollHandle,
    input: Entity<InputState>,
}

pub struct Message {
    pub text: SharedString,
    pub is_me: bool,
}

impl ChatView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            InputState::new(window, cx)
                .auto_grow(1, 5)
                .placeholder("Type a message...")
        });

        let messages = vec![
            Message {
                text: "Hello".into(),
                is_me: false,
            },
            Message {
                text: "Hi!".into(),
                is_me: true,
            },
        ];

        let mut this = Self {
            messages,
            item_sizes: Rc::new(vec![]),
            scroll_handle: VirtualListScrollHandle::new(),
            input,
        };

        this.update_item_sizes();

        this
    }

    fn update_item_sizes(&mut self) {
        self.item_sizes = Rc::new(
            self.messages
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
                            <div class="font-semibold">{"Alice"}</div>
                            <div class="text-xs text-zinc-500">{"online"}</div>
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
                                        let msg = &view.messages[ix];

                                        div()
                                            .w_full()
                                            .h(view.item_sizes[ix].height)
                                            .px_4()
                                            .py_2()
                                            .child(msg.text.clone())
                                    })
                                    .collect()
                            },
                        )
                        .track_scroll(&self.scroll_handle)
                    }
                </div>

                <div class="p-3 border-t border-zinc-800">
                    {Input::new(&self.input)}
                </div>
            </div>
        }
    }
}
