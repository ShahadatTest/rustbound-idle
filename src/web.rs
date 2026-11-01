use crate::game::Game;
use crate::player::PurchaseOutcome;
use crate::rules::enemy_health_for_stage;
use crate::save::LoadError;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Game)]
pub struct WebGame {
    inner: Game,
}

#[wasm_bindgen(js_class = Game)]
impl WebGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebGame {
        WebGame { inner: Game::new() }
    }

    pub fn click(&mut self) {
        self.inner.click();
    }

    pub fn tick(&mut self, seconds: u32) {
        self.inner.tick(seconds);
    }

    #[wasm_bindgen(js_name = buyClickUpgrade)]
    pub fn buy_click_upgrade(&mut self) -> String {
        match self.inner.buy_click_upgrade() {
            PurchaseOutcome::Purchased => String::from("purchased"),
            PurchaseOutcome::NotEnoughGold => String::from("notEnoughGold"),
        }
    }

    #[wasm_bindgen(js_name = stateJson)]
    pub fn state_json(&self) -> String {
        let snapshot = Snapshot::of(&self.inner);

        serde_json::to_string(&snapshot).expect("a Snapshot can always be written as JSON")
    }

    #[wasm_bindgen(js_name = saveJson)]
    pub fn save_json(&self) -> String {
        self.inner.save_json()
    }

    #[wasm_bindgen(js_name = fromJson)]
    pub fn from_json(json: &str) -> Result<WebGame, String> {
        match Game::from_saved_json(json) {
            Ok(inner) => Ok(WebGame { inner }),
            Err(LoadError::Unreadable) => Err(String::from("That save could not be read.")),
            Err(LoadError::UnsupportedVersion(version)) => Err(format!(
                "That save is version {version}, and this game cannot read it."
            )),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Snapshot {
    stage: u32,
    gold: u32,
    xp: u32,
    xp_needed: u32,
    level: u32,
    click_damage: u32,
    click_upgrade_level: u32,
    click_upgrade_cost: u32,
    enemy_name: String,
    enemy_health: u32,
    enemy_max_health: u32,
}

impl Snapshot {
    fn of(game: &Game) -> Snapshot {
        Snapshot {
            stage: game.stage,
            gold: game.player.gold,
            xp: game.player.xp,
            xp_needed: game.player.xp_needed(),
            level: game.player.level,
            click_damage: game.player.click_damage(),
            click_upgrade_level: game.player.click_upgrade_level,
            click_upgrade_cost: game.player.click_upgrade_cost(),
            enemy_name: game.enemy.name.clone(),
            enemy_health: game.enemy.health,
            enemy_max_health: enemy_health_for_stage(game.stage),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_game_reports_its_starting_state() {
        let game = WebGame::new();

        assert_eq!(
            game.state_json(),
            r#"{"stage":1,"gold":0,"xp":0,"xpNeeded":20,"level":1,"clickDamage":5,"clickUpgradeLevel":0,"clickUpgradeCost":10,"enemyName":"Training Slime","enemyHealth":20,"enemyMaxHealth":20}"#
        );
    }

    #[test]
    fn a_purchase_answers_in_words() {
        let mut game = WebGame::new();

        assert_eq!(game.buy_click_upgrade(), "notEnoughGold");

        game.tick(100);

        assert_eq!(game.buy_click_upgrade(), "purchased");
    }

    #[test]
    fn a_save_crosses_the_border_and_back() {
        let mut game = WebGame::new();
        game.tick(600);

        let save = game.save_json();
        let restored = WebGame::from_json(&save).expect("the save should load");

        assert_eq!(restored.state_json(), game.state_json());
    }

    #[test]
    fn nonsense_is_refused_at_the_border() {
        assert_eq!(
            WebGame::from_json("banana").err(),
            Some(String::from("That save could not be read."))
        );
    }
}
