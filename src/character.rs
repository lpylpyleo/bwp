use std::{default, error::Error, fs::File, io::BufReader};

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    #[serde(default)]
    pub level: u8,
    pub atk: i16,
    pub health: i16,
    #[serde(default)]
    pub state: CharacterState,
}

#[derive(Debug,Default, Serialize, Deserialize)]
pub enum CharacterState {
    #[default]
    None,
    Died,
}

impl Character {
    pub fn from_json() -> Result<Vec<Character>, Box<dyn Error>> {
        let file = File::open("src/models/characters.json")?;
        let reader = BufReader::new(file);
        let mut cs: Vec<Character> = serde_json::from_reader(reader)?;
        let mut rng = thread_rng();
        cs.shuffle(&mut rng);
        Ok(cs)
    }
}
