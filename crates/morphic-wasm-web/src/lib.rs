mod utils;

use std::{collections::BTreeMap, sync::{Arc, Mutex, OnceLock}};

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::{Window, WindowId}
};

use crate::render::CoreWindow;

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
    event_loop: Arc<OnceLock<EventLoop<()>>>,
    create_canvas: JsValue,
    core_windows: Arc<Mutex<BTreeMap<WindowId,Arc<CoreWindow>>>>
}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn new(obj: &Object) -> Result<Self, JsValue> {
        let get = |a: &str| Reflect::get(obj, &JsValue::from_str(a));
        let create_canvas = get("createCanvas")?;
        Ok(Self {
            _private: (),
            event_loop: Default::default(),
            create_canvas,
            core_windows: Default::default(),
        })
    }
}