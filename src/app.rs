use blenderui::prelude::*;
use std::path::PathBuf;
use time::Time;
use crate::components::chat::ChatProps;
use crate::{theme, utils};
use crate::components::conversation::Conversation;
use crate::components::burn::Burn;
use crate::backend::event::Event;
use crate::backend::init;
use crate::pages::stepper::Stepper;
use crate::backend::session::Session;
use blenderui::UiSender;

fn sample_chats() -> Vec<ChatProps> {
    let chat = |title: &'static str, content: &'static str, (h, m): (u8, u8), pinned, unread| ChatProps {
        icon_path: PathBuf::new(),
        title: title.into(),
        content: content.into(),
        time: Time::from_hms(h, m, 0).unwrap(),
        pinned,
        unread,
    };
    vec![
        chat("Alice", "See you tomorrow!", (9, 41), true, true),
        chat("Bob", "A very very very very very very very very very very very very very very very very long message", (18, 5), false, false),
        chat("Carol", "Thanks, that works for me", (14, 22), false, true),
    ]
}

#[component]
pub fn App(ui: UiSender) {
    let session = use_signal(Session::default());
    let burn_code = use_signal(None::<String>);

    use_effect!(move || {
        if let Some(mut events) = init::start() {
            std::thread::spawn(move || {
                while let Some(event) = events.blocking_recv() {
                    ui.send(move |_| {
                        match (session.get(), &event) {
                            (Session::Pairing(code), Event::Connected) => burn_code.set(Some(code)),
                            (_, Event::LoggedOut) => burn_code.set(None),
                            _ => {}
                        }
                        session.update(|s| *s = std::mem::take(s).next(event));
                    });
                }
            });
        }
    }, []);

    let chats = sample_chats();
    let selected = use_signal(0usize);
    let current = chats[selected.get()].title.clone();
    let count = chats.len();
    let cycle = move |e: &KeyDownEvent| {
        let m = e.keystroke.modifiers;
        if m.control && e.keystroke.key.to_ascii_lowercase().ends_with("tab") {
            selected.set((selected.get() + if m.shift { count - 1 } else { 1 }) % count);
        }
    };

    let stepper = |session: Session| rsx! {
      <body class="m-0 size-full" style={utils::style(&[("background-color", "background"), ("color", "foreground")])}>
        <Stepper session={session} />
      </body>
    };

    let burn = |code: Option<String>| match code {
        Some(code) => rsx! { <div class="absolute inset-0 pointer-events-none"><Burn data={SharedString::from(code)} /></div> },
        None => rsx! { <div /> },
    };

    let offline_banner = |offline: bool| if offline {
        rsx! { <div class="absolute top-0 inset-x-0 z-10 py-1 text-center text-xs" style={utils::style(&[("background-color", "secondary.background"), ("color", "muted.foreground")])}>"Reconnecting..."</div> }
    } else {
        rsx! { <div /> }
    };

    let chats_view = |burn_code: Option<String>, offline: bool| rsx! {
      <body class="m-0 size-full flex flex-row relative" onKeyDown={move |e| cycle(e)} style={utils::style(&[("background-color", "background"), ("color", "foreground")])}>
        <div class="w-96 shrink-0 flex flex-col overflow-hidden border-r" style={utils::style(&[("border-color", "border")])}>
          <div class="flex items-center justify-between px-4 py-3">
            <span class="text-lg font-semibold">"Chats"</span>
            <div class={format!("size-8 rounded-full flex items-center justify-center cursor-pointer hover:bg-[{}]", theme::color("secondary.active.background"))}>
              <img src="public/icons/search.svg" width="16" height="16" />
            </div>
          </div>
          {for (i, c) in chats.into_iter().enumerate() {
            <div class={format!("mx-2 my-0.5 rounded-lg {}", if selected.get() == i { format!("bg-[{}]", theme::color("list.active.background")) } else { format!("hover:bg-[{}]", theme::color("list.hover.background")) })} onClick={move || selected.set(i)}>
              <chat {...c} />
            </div>
          }}
        </div>
        <div class="flex-1 min-w-0">
          <Conversation name={current} chat={selected.get()} />
        </div>
        {offline_banner(offline)}
        {burn(burn_code)}
      </body>
    };

    rsx! {
        <div class="size-full">
            {match session.get() {
                Session::Online { offline } => chats_view(burn_code.get(), offline),
                other => stepper(other),
            }}
        </div>
    }
}
