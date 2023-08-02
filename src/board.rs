use crate::game::Color;

type Row = [Option<Piece>; Board::SIZE];

pub struct Board([Row; Board::SIZE]);

impl Board {
    pub const SIZE: usize = 8;

    fn empty_row() -> Row {
        [None; Self::SIZE]
    }

    fn pawns_row(color: Color) -> Row {
        let pawn = Piece::new(PieceKind::Pawn, color);
        [Some(pawn); Self::SIZE]
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
}

impl Default for Board {
    fn default() -> Self {
        Self([
            // row of white royals,
            Self::royals_row(Color::White),

            // row of white pawns,
            Self::pawns_row(Color::White),

            // 4 rows of no pieces
            Self::empty_row(),
            Self::empty_row(),
            Self::empty_row(),
            Self::empty_row(),

            // row of black pawns,
            Self::royals_row(Color::Black),

            // row of black royals,
            Self::pawns_row(Color::Black),
        ])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Piece {
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
