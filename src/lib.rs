use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Calculator;

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen]
    pub fn new() -> Calculator {
        Calculator {}
    }

    #[wasm_bindgen]
    pub fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }

    #[wasm_bindgen]
    pub fn subtract(&self, a: f64, b: f64) -> f64 {
        a - b
    }

    #[wasm_bindgen]
    pub fn multiply(&self, a: f64, b: f64) -> f64 {
        a * b
    }

    #[wasm_bindgen]
    pub fn divide(&self, a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("Cannot divide by zero".to_string())
        } else {
            Ok(a / b)
        }
    }
}
