use blenderui::prelude::*;
use std::path::PathBuf;
use time::Time;

mod components;
use components::chat::chat;

struct Demo;

impl Render for Demo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        rsx! {
            <body m-0 min-h-screen flex flex-col bg-zinc-900 text-white>
                <chat icon_path={PathBuf::new()} title={"Alice".to_string()} content={"See you tomorrow!".to_string()} time={Time::from_hms(9, 41, 0).unwrap()} pinned={true} unread={true} />
                <chat icon_path={PathBuf::new()} title={"Bob".to_string()} content={"A very long message that should get truncated with an ellipsis at the edge of the row".to_string()} time={Time::from_hms(18, 5, 0).unwrap()} pinned={false} unread={false} />
            </body>
        }
    }
}

fn main() {
    Application::new().gpu(true).run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Demo)).unwrap();
    });
}
