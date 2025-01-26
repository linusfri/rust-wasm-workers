mod utils;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-workers!");
}

#[wasm_bindgen]
pub fn return_arg(arg: String) -> String {
    arg
}


#[wasm_bindgen]
pub fn return_obj(obj: JsValue) -> JsValue {
    let mut person: Person = serde_wasm_bindgen::from_value(obj).unwrap();

    person.name = format!("Hello, {}-{}!", person.name, person.name);
    serde_wasm_bindgen::to_value::<Person>(&person).unwrap()
}