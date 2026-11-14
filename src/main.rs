use blenderui::prelude::*;

mod app;
use app::App;
pub mod components;
mod theme;

const THEME: &str = include_str!("../themes/theme.json");

struct Demo;

impl Render for Demo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        rsx! {
            <html class="size-full"><App /></html>
        }
    }
}

fn main() {
    blenderui::load_theme(THEME).expect("themes/theme.json");
    blenderui::set_theme("WhatsApp Dark");

    Application::new().gpu(true).run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Demo)).unwrap();
    });
}
