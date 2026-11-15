use blenderui::prelude::*;

use crate::components::qr::Qr;
use crate::backend::session::Session;

#[component]
pub fn Stepper(session: Session) {
    rsx! {
        <div class="size-full flex items-center justify-center">
            {match session {
                Session::Pairing(code) => rsx! { <Qr data={SharedString::from(code)} /> },
                Session::Failed(reason) => rsx! { <span>{reason}</span> },
                _ => rsx! { <span>"Starting..."</span> },
            }}
        </div>
    }
}
