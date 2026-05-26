use gpui::*;
use gpui_rsx::rsx;

struct CounterView {
    count: i32,
}

impl Render for CounterView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        rsx! {
            <div class="flex flex-col gap-4 p-4 bg-black w-full h-full">
                <div class="flex flex-row">
                    // options
                    <div class="flex flex-col">
                    </div>
                    // people
                    <div class="flex flex-col">
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
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_cx| CounterView { count: 0 })
        });
    });
}
