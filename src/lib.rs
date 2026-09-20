mod combat;
mod enemy;
mod game;
mod player;
mod progression;
mod upgrades;

use wasm_bindgen::prelude::*;

pub use game::Game;

#[wasm_bindgen(js_name = Game)]
pub struct WebGame {
    inner: Game,
}

#[wasm_bindgen(js_class = Game)]
impl WebGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Game::default(),
        }
    }

    #[wasm_bindgen(js_name = fromJson)]
    pub fn from_json(json: &str) -> Result<WebGame, JsValue> {
        Game::from_saved_json(json)
            .map(|inner| Self { inner })
            .map_err(|message| JsValue::from_str(&message))
    }

    pub fn click(&mut self) {
        self.inner.click();
    }

    pub fn tick(&mut self, seconds: u32) {
        self.inner.tick(seconds);
    }

    #[wasm_bindgen(js_name = buyClickUpgrade)]
    pub fn buy_click_upgrade(&mut self) -> bool {
        self.inner.buy_click_upgrade()
    }

    #[wasm_bindgen(js_name = buyIdleUpgrade)]
    pub fn buy_idle_upgrade(&mut self) -> bool {
        self.inner.buy_idle_upgrade()
    }

    #[wasm_bindgen(js_name = stateJson)]
    pub fn state_json(&self) -> Result<String, JsValue> {
        self.inner
            .state_json()
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = saveJson)]
    pub fn save_json(&self) -> Result<String, JsValue> {
        self.inner
            .save_json()
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }
}

impl Default for WebGame {
    fn default() -> Self {
        Self::new()
    }
}
