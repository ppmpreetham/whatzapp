use blenderui::prelude::*;
use strum::IntoStaticStr;
pub mod file;

use crate::{theme, utils::cx};

const KEYFRAMES: &str = r#"
@keyframes wz-pop { from { transform: translateY(6px) scale(0.97); } to { transform: translateY(0px) scale(1); } }
@keyframes wz-fade { from { opacity: 0.4; } to { opacity: 1; } }
@keyframes wz-dots { 0% { opacity: 0.35; transform: translateY(0px); } 30% { opacity: 1; transform: translateY(-2px); }
                     60% { opacity: 0.35; transform: translateY(0px); } 100% { opacity: 0.35; transform: translateY(0px); } }
@keyframes wz-swap-in { from { opacity: 0; transform: translateY(4px); filter: blur(2px); } to { opacity: 1; transform: translateY(0px); filter: blur(0px); } }
"#;

const BUBBLE: &str = "max-w-[78%] rounded-[18px] transition-[border-radius] duration-[250ms] ease-[cubic-bezier(0.22,1,0.36,1)]";
const POP: &str = "animate-[wz-pop_200ms_cubic-bezier(0.22,1,0.36,1)_backwards,wz-fade_120ms_ease-out_backwards]";

#[derive(Clone, Copy, PartialEq, IntoStaticStr)]
pub enum Delivery {
    Sent,
    Delivered,
    Read,
}

#[derive(Clone)]
pub struct Msg {
    pub id: u32,
    pub text: SharedString,
    // is mine or not
    pub mine: bool,
    pub fresh: bool,
}

fn row(mine: bool, starts_run: bool) -> String {
    cx(&["flex flex-col", if mine { "items-end" } else { "items-start" }, if starts_run { "mt-1.5" } else { "mt-0.5" }])
}

fn colors(mine: bool) -> &'static str {
    if mine {
        "bg-[color:var(--wz-out)] text-[color:var(--wz-outfg)] origin-bottom-right"
    } else {
        "bg-[color:var(--wz-in)] origin-bottom-left"
    }
}

fn tail(mine: bool, last_of_run: bool) -> &'static str {
    match (last_of_run, mine) {
        (false, _) => "",
        (true, true) => "rounded-br-[6px]",
        (true, false) => "rounded-bl-[6px]",
    }
}

#[component]
pub fn Thread(messages: Vec<Msg>, status: Delivery, typing: bool) {
    let last_sent = messages.iter().rev().find(|m| m.mine).map(|m| m.id);
    let last_mine = messages.last().map(|m| m.mine);
    let label: &'static str = status.into();
    let sides: Vec<bool> = messages.iter().map(|m| m.mine).collect();

    rsx! {
        <div class="flex-1 min-h-0 flex flex-col overflow-y-auto px-4 py-3" data-stick="always" style={theme::vars()}>
            <style>{KEYFRAMES}</style>
            <div class="mt-auto flex flex-col">
                {for (i, m) in messages.into_iter().enumerate() {
                    <div key={format!("row-{}", m.id)} class={row(m.mine, i > 0 && sides[i - 1] != m.mine)}>
                        <div class={cx(&[
                            BUBBLE,
                            colors(m.mine),
                            tail(m.mine, sides.get(i + 1) != Some(&m.mine)),
                            if m.fresh { POP } else { "" },
                            "px-3 py-2 text-sm leading-5 min-w-0 break-words",
                        ])}>{m.text.clone()}</div>
                        {if Some(m.id) == last_sent {
                            rsx! {
                                <div class="pt-0.5 pr-1 text-xs leading-4 text-[color:var(--wz-muted)]">
                                    <span key={format!("status-{}", label)}
                                          class="block animate-[wz-swap-in_150ms_ease-in-out_backwards]">{label}</span>
                                </div>
                            }
                        } else { div() }}
                    </div>
                }}
                {if typing {
                    rsx! {
                        <div key="typing" class={row(false, last_mine == Some(true))}>
                            <div class={cx(&[BUBBLE, colors(false), tail(false, true), POP, "flex flex-row items-center gap-1 px-3 py-3.5"])}>
                                {for delay in [0, 180, 360] {
                                    <span class={format!(
                                        "size-1.5 rounded-full bg-[color:var(--wz-muted)] animate-[wz-dots_1200ms_ease-in-out_{delay}ms_infinite]"
                                    )} />
                                }}
                            </div>
                        </div>
                    }
                } else { div() }}
            </div>
        </div>
    }
}
