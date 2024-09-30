use super::player::Player;

#[derive(Debug)]
pub struct Game {
    pub round: u8,
    pub player_a: Player,
    pub player_b: Player,
    pub state: GameState,
}

#[derive(Debug)]
pub enum GameState{
    WaitingLevelUp,
    WaitingPlayerInput,
}

impl Game {
    pub fn next_round(&mut self){
        self.round += 1;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            round: 0,
            player_a: Player {
                name: "A".to_string(),
                ..Default::default()
            },
            player_b: Player {
                name: "B".to_string(),
                ..Default::default()
            },
            state:GameState::WaitingLevelUp,
        }
    }
}
