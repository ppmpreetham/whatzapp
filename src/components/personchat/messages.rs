use blenderui::prelude::*;
use blenderui::vlist::{chat_list, theme_palette, ChatList, ChatStyle, ListState, Message, Options, VecSource};

pub type MessageList = Entity<ListState<VecSource, ChatStyle>>;

type Line = (u16, u32, &'static str);

// TODO: one backend is done, we can remove these sample datas
const BASE: u32 = 1_700_000_000;

const ALICE: &[Line] = &[
    (2, 0, "Hey! Are we still on for tomorrow?"),
    (1, 60, "Yes, see you at 10."),
    (1, 90, "I'll bring the documents, and the laptop in case we need to fix anything on the spot."),
    (2, 400, "Perfect. See you tomorrow!"),
];
const BOB: &[Line] = &[
    (2, 0, "A very very very very very very very very very very very very very very very very long message that needs to wrap over a few lines so the row is taller than the rest."),
    (1, 30, "ok"),
    (2, 3600, "Did you get my last message?"),
];
const CAROL: &[Line] = &[
    (1, 0, "Can you move the call to 3pm?"),
    (2, 120, "Thanks, that works for me"),
];

pub fn sample_lists(cx: &mut App) -> Vec<MessageList> {
    [ALICE, BOB, CAROL].into_iter().map(|lines| new_list(cx, lines)).collect()
}

fn new_list(cx: &mut App, lines: &[Line]) -> MessageList {
    let opts = Options { start_at_end: true, ..Options::default() };
    let mut list = ChatList::from_messages(vec![], theme_palette(), 600.0, 400.0, opts);
    for (i, &(sender, dt, text)) in lines.iter().enumerate() {
        // TODO: here asw
        list.push(Message {
            id: i as u64 + 1,
            time: BASE + dt,
            sender,
            flags: 0,
            media_h: 0,
            text: text.to_string(),
            media: String::new(),
            ..Message::default()
        });
    }
    cx.new(|cx| ListState::new(list, cx))
}

#[component]
pub fn Messages(list: MessageList) {
    rsx! {
        <div class="flex-1 min-h-0 w-full">
            {chat_list(&list)}
        </div>
    }
}
