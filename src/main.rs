use blenderui::prelude::*;

mod app;
use app::App;
pub mod components;

struct Demo;

impl Render for Demo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        rsx! {
            <html class="size-full"><App /></html>
        }
    }
}

fn main() {
    Application::new().gpu(true).run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Demo)).unwrap();
    });
}
