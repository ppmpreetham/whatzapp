use blenderui::prelude::*;

mod app;
use app::App;

pub mod components;
pub mod utils;
pub mod pages;

mod theme;
mod backend;

/// WhatsApp Light and WhatsApp Dark (bui theme file, see bui/docs/theming.md).
const THEME: &str = include_str!("../themes/theme.json");

struct Demo {
    ui: blenderui::UiSender,
}

impl Render for Demo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        rsx! {
            <html class="size-full"><App ui={self.ui.clone()} /></html>
        }
    }
}

fn main() {
    blenderui::load_theme(THEME).expect("themes/theme.json");
    blenderui::set_theme("WhatsApp Dark");

    Application::new().gpu(true).run(|cx| {
        let ui = cx.ui_sender();
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Demo { ui }))
        .unwrap();
    });
}
