use blenderui::prelude::*;

#[component]
pub fn SingleMessage(text: SharedString) {
  rsx!{
    <div class="">
      {text}
    </div>
  }
}
