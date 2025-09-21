mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn preinit(){
    utils::set_panic_hook();
}