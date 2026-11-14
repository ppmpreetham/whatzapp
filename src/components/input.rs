use blenderui::prelude::*;
use crate::utils;

#[component]
pub fn MessageInput() {
    let text = use_signal(String::new());
    let has_content = !text.get().trim().is_empty();

    rsx! {
        <div class="relative p-3" style={utils::style(&[("background-color", "secondary.background")])}>
            <div class="flex items-end gap-2">
                <div class="flex-1 min-w-0 rounded-lg" style={utils::style(&[("background-color", "secondary.active.background")])}>
                    <textarea
                        class="w-full p-2 bg-transparent border-none outline-none field-sizing-content max-h-32" style={utils::style(&[("color", "foreground"), ("caret-color", "foreground")])}
                        placeholder="Type a message"
                        rows="1"
                        value={text.get()}
                        onInput={move |e| text.set(e.value.clone())}
                    />
                </div>
                <div
                    class={if has_content { "px-4 py-2 rounded-full cursor-pointer" } else { "px-4 py-2 rounded-full cursor-not-allowed opacity-50" }} style={utils::style(&[("background-color", "primary.background"), ("color", "primary.foreground")])}
                    onClick={move || if has_content { text.set(String::new()) }}
                >
                    "Send"
                </div>
            </div>
        </div>
    }
}
