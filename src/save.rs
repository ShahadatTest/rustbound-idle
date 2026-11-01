use crate::enemy::Enemy;
use crate::game::Game;
use crate::rules::enemy_health_for_stage;
use serde::{Deserialize, Serialize};

const SAVE_VERSION: u32 = 1;

#[derive(Debug, PartialEq)]
pub enum LoadError {
    Unreadable,
    UnsupportedVersion(u32),
    Impossible,
}

#[derive(Serialize, Deserialize)]
struct PlayerSave {
    gold: u32,
    xp: u32,
    level: u32,
    click_upgrade_level: u32,
}

#[derive(Serialize, Deserialize)]
struct SaveData {
    save_version: u32,
    stage: u32,
    player: PlayerSave,
    enemy_health: u32,
}

impl Game {
    pub fn save_json(&self) -> String {
        let save = SaveData {
            save_version: SAVE_VERSION,
            stage: self.stage,
            player: PlayerSave {
                gold: self.player.gold,
                xp: self.player.xp,
                level: self.player.level,
                click_upgrade_level: self.player.click_upgrade_level,
            },
            enemy_health: self.enemy.health,
        };

        serde_json::to_string(&save).expect("a save made of numbers should always serialize")
    }

    pub fn from_saved_json(json: &str) -> Result<Game, LoadError> {
        let save: SaveData = match serde_json::from_str(json) {
            Ok(save) => save,
            Err(_) => return Err(LoadError::Unreadable),
        };

        if save.save_version != SAVE_VERSION {
            return Err(LoadError::UnsupportedVersion(save.save_version));
        }

        if save.stage == 0 || save.player.level == 0 {
            return Err(LoadError::Impossible);
        }

        let mut game = Game::new();

        game.stage = save.stage;
        game.player.gold = save.player.gold;
        game.player.xp = save.player.xp;
        game.player.level = save.player.level;
        game.player.click_upgrade_level = save.player.click_upgrade_level;

        game.enemy = Enemy::for_stage(save.stage);
        game.enemy.health = save.enemy_health.min(enemy_health_for_stage(save.stage));

        game.player.add_xp(0);

        Ok(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_game_saves_its_starting_numbers() {
        let json = Game::new().save_json();

        assert_eq!(
            json,
            r#"{"save_version":1,"stage":1,"player":{"gold":0,"xp":0,"level":1,"click_upgrade_level":0},"enemy_health":20}"#
        );
    }

    #[test]
    fn a_loaded_game_rebuilds_the_enemy_from_the_rules() {
        let mut game = Game::new();
        game.tick(600);

        let restored = Game::from_saved_json(&game.save_json()).expect("the save should load");

        assert_eq!(restored.stage, game.stage);
        assert_eq!(restored.enemy.health, game.enemy.health);
        assert_eq!(restored.enemy.gold_reward, game.enemy.gold_reward);
        assert_eq!(restored.enemy.xp_reward, game.enemy.xp_reward);
    }

    #[test]
    fn nonsense_is_not_a_save() {
        assert!(matches!(
            Game::from_saved_json("definitely not a save"),
            Err(LoadError::Unreadable)
        ));
    }

    #[test]
    fn a_save_with_missing_pieces_is_not_a_save() {
        let json = r#"{"save_version":1,"stage":4}"#;

        assert!(matches!(
            Game::from_saved_json(json),
            Err(LoadError::Unreadable)
        ));
    }

    #[test]
    fn a_save_from_another_version_is_refused() {
        let json = r#"{
            "save_version": 99,
            "stage": 1,
            "player": { "gold": 0, "xp": 0, "level": 1, "click_upgrade_level": 0 },
            "enemy_health": 20
        }"#;

        assert!(matches!(
            Game::from_saved_json(json),
            Err(LoadError::UnsupportedVersion(99))
        ));
    }

    #[test]
    fn an_enemy_cannot_have_more_health_than_its_stage_allows() {
        let mut game = Game::new();
        game.enemy.health = 4_000_000_000;

        let restored = Game::from_saved_json(&game.save_json()).expect("the save should load");

        assert_eq!(restored.enemy.health, enemy_health_for_stage(1));
    }

    #[test]
    fn leftover_xp_is_turned_into_levels() {
        let mut game = Game::new();
        game.player.xp = 1_000;

        let restored = Game::from_saved_json(&game.save_json()).expect("the save should load");

        assert!(restored.player.level > 1);
        assert!(restored.player.xp < restored.player.xp_needed());
    }

    #[test]
    fn stage_zero_is_not_a_place() {
        let mut game = Game::new();
        game.stage = 0;

        assert!(matches!(
            Game::from_saved_json(&game.save_json()),
            Err(LoadError::Impossible)
        ));
    }

    #[test]
    fn level_zero_is_not_a_level() {
        let mut game = Game::new();
        game.player.level = 0;

        assert!(matches!(
            Game::from_saved_json(&game.save_json()),
            Err(LoadError::Impossible)
        ));
    }

    #[test]
    fn a_great_deal_of_gold_is_still_just_gold() {
        let mut game = Game::new();
        game.player.gold = 11_000_000;

        let restored = Game::from_saved_json(&game.save_json()).expect("the save should load");

        assert_eq!(restored.player.gold, 11_000_000);
    }
}
