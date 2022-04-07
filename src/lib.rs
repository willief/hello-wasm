use wasm_bindgem::prelude::*;

#[wasm_bindgem]
extern  {
    pub fn alert(s: &str);
}

#[wasm_bindgem]
pub fn greet(name &str) {
    alert(&format!("Hello, {}!", name))
}