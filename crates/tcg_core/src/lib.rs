mod command;
mod engine;
mod event;
mod model;

use command::Command;
use engine::CoreEngine;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct Engine {
    core: CoreEngine,
}


#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Engine {
        console_error_panic_hook::set_once();

        Engine {
            core: CoreEngine::new(),
        }
    }

    pub fn get_state(&self) -> JsValue {
        to_value(&self.core.state).unwrap()
    }

    pub fn dispatch(&mut self, command: JsValue) -> Result<JsValue, JsValue> {
        let command: Command = from_value(command)
            .map_err(|err| JsValue::from_str(&format!("Invalid command: {}", err)))?;

        let events = self
            .core
            .dispatch(command)
            .map_err(|err| JsValue::from_str(&err))?;

        to_value(&events)
            .map_err(|err| JsValue::from_str(&format!("Failed to serialize events: {}", err)))
    }
}