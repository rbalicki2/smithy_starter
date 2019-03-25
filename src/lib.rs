#![feature(
  proc_macro_hygiene,
  slice_patterns,
  custom_attribute,
  extern_crate_item_prelude,
  bind_by_move_pattern_guards
)]

use wasm_bindgen::prelude::*;
use web_sys::{
  Document,
  Element,
};

#[wasm_bindgen]
pub fn start(root_element: web_sys::Element) {
  let app = smithy::smd!(
    <div>Welcome to Smithy!</div>
  );

  smithy::mount(Box::new(app), root_element);
}
