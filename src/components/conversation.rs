use blenderui::prelude::*;

use crate::components::thread::{Delivery, Msg, Thread};
use crate::{theme, utils::{cx, initial}};

const ROUND: &str = "shrink-0 rounded-full flex items-center justify-center transition-[transform,background-color] duration-150 active:scale-95";

#[component]
pub fn Conversation(name: SharedString, chat: usize) {
    // temp here
    let seed = |id, mine, text: &'static str| Msg { id, text: text.into(), mine, fresh: false };
    let threads = use_signal(vec![
        vec![
        seed(1, false, "Are we still on for Saturday?"),
        seed(2, true, "Yes! Trail at 9?"),
        seed(3, false, "Perfect. I'll bring coffee"),
        seed(4, false, "The good kind this time"),
        seed(5, true, "You're the best"),
        ],
        vec![
            seed(1, true, "Did you get my last message?"),
            seed(2, false, "Just saw it, sorry"),
            seed(3, false, "Sending the files now"),
        ],
        vec![
            seed(1, false, "Can you move the call to 3pm?"),
            seed(2, true, "Thanks, that works for me"),
        ],
    ]);
    let messages = threads.get()[chat].clone();

    let draft = use_signal(String::new());
    let shift = use_signal(false);
    let status = use_signal(vec![Delivery::Read; 3]);

    let ready = !draft.get().trim().is_empty();

    let send = move || {
        let text = draft.get().trim().to_string();
        if text.is_empty() {
            return;
        }
        threads.update(|t| {
            let id = t[chat].last().map_or(1, |m| m.id + 1);
            t[chat].push(Msg { id, text: text.into(), mine: true, fresh: true });
        });
        draft.set(String::new());
        status.update(|s| s[chat] = Delivery::Delivered);
    };

    let initial = initial(&name);

    rsx! {
        <div class="flex flex-col w-full h-full bg-[color:var(--wz-bg)] text-[color:var(--wz-fg)]" style={theme::vars()}>
            <header class="relative z-[2] flex items-center gap-3 px-4 py-3 bg-[color:var(--wz-surface)]">
                <span class="shrink-0 size-9 rounded-full flex items-center justify-center bg-[color:var(--wz-chip)] text-[15px] font-medium">{initial}</span>
                <div class="flex-1 min-w-0 flex flex-col">
                    <span class="text-[15px] leading-5 font-medium truncate">{name.clone()}</span>
                    <span class="text-xs leading-4 text-[color:var(--wz-muted)]">"online"</span>
                </div>
                <div class="shrink-0 flex gap-1 -mr-1.5">
                    <div class="group relative flex">
                        <div class={cx(&[ROUND, "size-8 cursor-pointer hover:bg-[color:var(--wz-chip)]"])} key="audio">
                            <img src="public/icons/phone.svg" width="16" height="16" />
                        </div>
                        <div class="absolute top-10 right-0 px-2.5 py-1.5 rounded-lg bg-[color:var(--wz-popover)] text-[color:var(--wz-fg)] text-xs leading-4 whitespace-nowrap z-[3] opacity-0 scale-[0.98] transition-[opacity,scale] duration-[50ms] ease-out group-hover:opacity-100 group-hover:scale-100 group-hover:duration-150 group-hover:delay-[80ms]">"Audio call"</div>
                    </div>
                </div>
            </header>
            <div class="h-px shrink-0 bg-[color:var(--wz-border)]" />

            <Thread messages={messages} status={status.get()[chat]} typing={false} />

            <div class="flex items-end gap-2 px-4 pt-2 pb-4 bg-[color:var(--wz-bg)]">
                <label class="flex-1 min-w-0 flex items-center min-h-10 px-3.5 rounded-[20px] bg-[color:var(--wz-surface)] text-[color:var(--wz-fg)] focus-within:bg-[color:var(--wz-chip)]">
                    <textarea class="flex-1 min-w-0 py-2.5 bg-transparent border-none outline-none resize-none text-sm leading-5 text-[color:var(--wz-fg)] field-sizing-content max-h-32" rows="1" live="true" placeholder="Type a message"
                              value={draft.get()}
                              onKeyDown={move |e| shift.set(e.keystroke.modifiers.shift)}
                              onInput={move |e| {
                                  let v = e.value.clone();
                                  // a trailing newline is Enter: send it, unless Shift was held
                                  if v.ends_with('\n') && !shift.get() {
                                      draft.set(v.trim_end().to_string());
                                      send();
                                  } else {
                                      draft.set(v);
                                  }
                              }} />
                </label>
                <div class={if ready { "shrink-0 size-10 rounded-full flex items-center justify-center transition-[transform,background-color] duration-150 active:scale-95 cursor-pointer bg-[color:var(--wz-accent)]" } else { "shrink-0 size-10 rounded-full flex items-center justify-center transition-[transform,background-color] duration-150 active:scale-95 cursor-default bg-[color:var(--wz-chip)]" }} onClick={move || send()}>
                    <img src={if ready { "public/icons/send-ready.svg" } else { "public/icons/send.svg" }} width="16" height="16" />
                </div>
            </div>
        </div>
    }
}
