use blenderui::prelude::*;

#[component]
pub fn MessageInput() {
    let text = use_signal(String::new());
    let has_content = !text.get().trim().is_empty();

    rsx! {
        <div class="relative p-3 bg-zinc-800">
            <div class="flex items-end gap-2">
                <div class="flex-1 min-w-0 bg-zinc-700 rounded-lg">
                    <textarea
                        class="w-full p-2 bg-transparent border-none outline-none field-sizing-content max-h-32 text-white"
                        placeholder="Type a message"
                        rows="1"
                        value={text.get()}
                        onInput={move |e| text.set(e.value.clone())}
                    />
                </div>
                <div
                    class={format!("px-4 py-2 rounded-full text-white {}", if has_content { "bg-green-500 hover:bg-green-600" } else { "bg-green-500/50" })}
                    onClick={move || if has_content { text.set(String::new()) }}
                >
                    "Send"
                </div>
            </div>
        </div>
    }
}
