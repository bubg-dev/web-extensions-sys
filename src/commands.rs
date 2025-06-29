//! Bindings to the `commands` API.

use wasm_bindgen::prelude::*;

use crate::EventTarget;

#[wasm_bindgen]
extern "C" {

  pub type Commands;

  #[wasm_bindgen(method, getter, js_name = onCommand)]
  pub fn on_command(this: &Commands) -> EventTarget;
}
