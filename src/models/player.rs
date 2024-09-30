use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;

use crate::character::*;
use crate::utils::*;

const CHARACTER_COUNT: u8 = 4;
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub health: u8,
    pub characters: Vec<Character>, // 4个
}

impl Player {
    pub fn level_up_characters(&mut self, points: u8) {
        for point_i in 0..points {
            // 找出当前等级最低的角色
            let min_level = self.characters.iter().map(|ch| ch.level).min().unwrap_or(0);

            // 允许升级的角色索引（等级最低的）
            let allowed_character_index: Vec<usize> = self
                .characters
                .iter()
                .enumerate()
                .filter(|(_, ch)| ch.level == min_level)
                .map(|(i, _)| i)
                .collect();

            // println!(
            //     "{} have {} points to allocate. Current point: {}",
            //     self.name,
            //     points,
            //     point_i + 1
            // );

            // println!(
            //     "Allowed characters for leveling up: {:?}",
            //     allowed_character_index
            // );

            let mut i: usize;
            loop {
                // 获取用户输入
                i = require_user_input();
                if i >= self.characters.len() || !allowed_character_index.contains(&i) {
                    println!(
                        "Invalid selection. Please choose from {:?}",
                        allowed_character_index
                    );
                } else {
                    // 找到有效角色，应用升级逻辑
                    self.characters[i].level += 1;
                    println!(
                        "Character '{}' leveled up! New level: {}",
                        self.characters[i].name, self.characters[i].level
                    );
                    break;
                }
            }
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            health: 30,
            characters: Character::from_json().unwrap(),
        }
    }
}
