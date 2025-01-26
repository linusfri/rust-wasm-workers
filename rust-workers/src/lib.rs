mod utils;

use rand::Rng;
use uuid::Uuid;
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

// Define the Size struct with width and height.
#[derive(Clone, Serialize, Deserialize)]
struct Size {
    width: f64,
    height: f64,
}

// Define the Position struct with x and y.
#[derive(Clone, Serialize, Deserialize)]
struct Position {
    x: f64,
    y: f64,
}

// Define the Panel struct with all fields from the TypeScript version.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Panel {
    uid: String,
    panel_area_uid: String,
    row: u32,
    column: u32,
    position: Position,
    size: Size,
    panel_area_size: Size,
    removed: bool,
    fill: String,
}

pub fn get_panels_four_corners(panels: &Vec<Panel>) -> Vec<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> {
    panels.into_iter().map(|panel| {
        let top_left = vec![panel.position.x, panel.position.y];
        let top_right = vec![panel.position.x + panel.size.width, panel.position.y];
        let bottom_right = vec![panel.position.x + panel.size.width, panel.position.y + panel.size.height];
        let bottom_left = vec![panel.position.x, panel.position.y + panel.size.height];
        
        (top_left, top_right, bottom_right, bottom_left)
    }).collect()
}

pub fn generate_alot_of_panels() -> Vec<Panel> {
    let mut panels = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..500_000 {
        panels.push(Panel {
            uid: Uuid::new_v4().to_string(),
            panel_area_uid: Uuid::new_v4().to_string(),
            row: rng.gen_range(0..10),
            column: rng.gen_range(0..10),
            position: Position {
                x: rng.gen_range(0.0..10.0),
                y: rng.gen_range(0.0..10.0),
            },
            size: Size {
                width: rng.gen_range(0.0..10.0),
                height: rng.gen_range(0.0..10.0),
            },
            panel_area_size: Size {
                width: rng.gen_range(0.0..100.0),
                height: rng.gen_range(0.0..100.0),
            },
            removed: false,
            fill: String::from("red"),
        });
    }

    panels
}

#[wasm_bindgen]
pub fn time_rust() {
    // let panels: Vec<Panel> = panels.iter().map(|panel| {
    //     serde_wasm_bindgen::from_value(panel.clone()).unwrap()
    // }).collect();
    let times_to_run = 10;
    let panels = generate_alot_of_panels();

    for _ in 0..times_to_run {
        get_panels_four_corners(&panels);
    }
}