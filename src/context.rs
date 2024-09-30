use std::{cell::RefCell, rc::Rc};

use crate::models::game::Game;

pub type BuildContext = Rc<RefCell<Context>>;

pub struct Context {
    pub game: Game,
}

impl Context {
    pub fn new(game: Game) -> BuildContext {
        Rc::new(RefCell::new(Context { game }))
    }
}
