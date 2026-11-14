use blenderui::prelude::*;

#[component]
pub fn Input() {
  rsx!{
    <div class="flex flex-row p-4">
      <input class="flex-1 border" />
    </div>
  }
}
