use blenderui::prelude::*;

mod app;
use app::App;
pub mod components;
pub mod utils;
mod theme;

use components::personchat::messages::{sample_lists, MessageList};

/// WhatsApp Light and WhatsApp Dark (bui theme file, see bui/docs/theming.md).
const THEME: &str = include_str!("../themes/theme.json");

struct Demo {
    lists: Vec<MessageList>,
}

impl Render for Demo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        rsx! {
            <html class="size-full"><App lists={self.lists.clone()} /></html>
        }
    }
}

fn main() {
    blenderui::load_theme(THEME).expect("themes/theme.json");
    blenderui::set_theme("WhatsApp Dark");

    Application::new().gpu(true).run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            let lists = sample_lists(cx);
            cx.new(|_| Demo { lists })
        })
        .unwrap();
    });
}
