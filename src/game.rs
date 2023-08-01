use std::fmt::{self, Display, Formatter};
use crate::{err::MoveError, mv::Move};

type Row = [Option<Piece>; Game::BOARD_SIZE];

pub struct Game {
    board: [Row; Game::BOARD_SIZE],
    current_player: Color,
}

impl Game {
    pub const BOARD_SIZE: usize = 8;

    pub fn new() -> Self {
        Game {
            board: [
                // row of white royals,
                Game::royals_row(Color::White),

                // row of white pawns,
                Game::pawns_row(Color::White),

                // 4 rows of no pieces
                Game::empty_row(),
                Game::empty_row(),
                Game::empty_row(),
                Game::empty_row(),

                // row of black pawns,
                Game::royals_row(Color::Black),

                // row of black royals,
                Game::pawns_row(Color::Black),
            ],

            current_player: Color::White,
        }
    }

    fn empty_row() -> Row {
        [None; Game::BOARD_SIZE]
    }

    fn pawns_row(color: Color) -> Row {
        let pawn = Piece::new(PieceKind::Pawn, color);
        [Some(pawn); Game::BOARD_SIZE]
    }

    fn royals_row(color: Color) -> Row {
        [
            Some(Piece::new(
                PieceKind::Rook,
                color,
            )),
            Some(Piece::new(
                PieceKind::Knight,
                color,
            )),
            Some(Piece::new(
                PieceKind::Bishop,
                color,
            )),
            Some(Piece::new(
                PieceKind::Queen,
                color,
            )),
            Some(Piece::new(
                PieceKind::King,
                color,
            )),
            Some(Piece::new(
                PieceKind::Bishop,
                color,
            )),
            Some(Piece::new(
                PieceKind::Knight,
                color,
            )),
            Some(Piece::new(
                PieceKind::Rook,
                color,
            )),
        ]
    }

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Piece {
    kind: PieceKind,
    color: Color,
}

impl Piece {
    pub fn new(kind: PieceKind, color: Color) -> Self {
        Self { kind, color }
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

#[derive(Clone, Copy, Debug, soccer::Display, Eq, soccer::Into, PartialEq)]
#[const_ty(&'static str)]
pub enum Color {
    #[const_val("white")]
    White,

    #[const_val("black")]
    Black,
}
