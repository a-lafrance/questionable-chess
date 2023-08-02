use std::fmt::{self, Display, Formatter};

use crate::{board::Board, err::MoveError, mv::Move};

#[derive(Default)]
pub struct Game {
    board: Board,
    current_player: Color,
}

impl Game {
    pub fn current_player(&self) -> Color {
        self.current_player
    }

    pub fn make_move(&mut self, _mv: Move) -> Result<(), MoveError> {
        todo!()
    }

    pub fn winner(&self) -> Option<Color> {
        todo!()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "board goes here")
    }
}

#[derive(Clone, Copy, Debug, Default, soccer::Display, Eq, soccer::Into, PartialEq)]
#[const_ty(&'static str)]
pub enum Color {
    #[const_val("white")]
    #[default]
    White,

    #[const_val("black")]
    Black,
}
