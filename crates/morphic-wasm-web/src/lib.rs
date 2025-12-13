mod utils;

use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, OnceLock},
};

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowAttributes, WindowId},
};

#[wasm_bindgen(module = "@portal-solutions/morphic-wasm-dep-rollup")]
extern "C" {
    type DOMTag;
    #[wasm_bindgen(constructor)]
    fn new_dom_tag(x: JsValue) -> DOMTag;
    #[wasm_bindgen(method, getter, js_name = "value")]
    fn get_wrapped(this: &DOMTag) -> JsValue;
}

#[wasm_bindgen(start)]
fn preinit() {
    utils::set_panic_hook();
}
pub mod render;
#[wasm_bindgen]
#[derive(Clone)]
pub struct Core {
    _private: (),
    base: morphic_winit_impl::Core,
    create_canvas: JsValue,
}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn new(obj: &Object) -> Result<Self, JsValue> {
        let get = |a: &str| Reflect::get(obj, &JsValue::from_str(a));
        let create_canvas = get("createCanvas")?;
        Ok(Self {
            _private: (),
            // event_loop: Default::default(),
            create_canvas,
            base: Default::default(),
            // core_windows: Default::default(),
        })
    }
}
