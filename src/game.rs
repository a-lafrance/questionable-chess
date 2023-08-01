use crate::{err::MoveError, mv::Move};

pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Game {}
    }

    pub fn current_player(&self) -> String {
        todo!()
    }

    pub fn make_move(&mut self, _mv: Move) -> Result<(), MoveError> {
        todo!()
    }

    pub fn winner(&self) -> Option<String> {
        todo!()
    }
}

#[derive(Clone, Copy, Debug, soccer::Display, Eq, soccer::Into, PartialEq, soccer::TryFrom)]
#[const_ty(char)]
pub enum PieceKind {
    #[const_val('P')]
    Pawn,
    #[const_val('R')]
    Rook,
    #[const_val('N')]
    Knight,
    #[const_val('B')]
    Bishop,
    #[const_val('Q')]
    Queen,
    #[const_val('K')]
    King,
}
