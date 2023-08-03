use std::{
    cmp,
    fmt::{self, Display, Formatter},
};

use crate::{
    err::MoveError,
    game::Color,
    mv::{self, Square},
};

type Row = [Option<Piece>; Board::SIZE];

pub struct Board([Row; Board::SIZE]);

impl Board {
    pub const BLACK_PAWN_START_ROW: usize = Board::SIZE - 2;
    pub const SIZE: usize = 8;
    pub const WHITE_PAWN_START_ROW: usize = 1;

    fn empty_row() -> Row {
        [None; Self::SIZE]
    }

    fn pawns_row(color: Color) -> Row {
        let pawn = Piece::new(PieceKind::Pawn, color);
        [Some(pawn); Self::SIZE]
    }

    fn royals_row(color: Color) -> Row {
        [
            Some(Piece::new(PieceKind::Rook, color)),
            Some(Piece::new(PieceKind::Knight, color)),
            Some(Piece::new(PieceKind::Bishop, color)),
            Some(Piece::new(PieceKind::Queen, color)),
            Some(Piece::new(PieceKind::King, color)),
            Some(Piece::new(PieceKind::Bishop, color)),
            Some(Piece::new(PieceKind::Knight, color)),
            Some(Piece::new(PieceKind::Rook, color)),
        ]
    }

    pub fn move_piece(
        &mut self,
        kind: PieceKind,
        color: Color,
        start: Square,
        end: Square,
    ) -> Result<Option<Piece>, MoveError> {
        let piece = match self.piece(start) {
            Some(piece) => {
                if piece.kind() != kind {
                    return Err(MoveError::WrongPieceKind);
                }

                if piece.color() != color {
                    return Err(MoveError::WrongPieceColor);
                }

                piece
            },

            None => return Err(MoveError::PieceNotFound),
        };

        // not sure if there's any better way to erase an option
        *self.piece_handle(start) = None;
        Ok(self.piece_handle(end).replace(piece))
    }

    pub fn piece(&self, sq: Square) -> Option<Piece> {
        self.0[sq.row][sq.col]
    }

    pub fn has_piece(&self, sq: Square) -> bool {
        self.piece(sq).is_some()
    }

    pub fn has_opposing_piece(&self, sq: Square, color: Color) -> bool {
        self.piece(sq)
            .map(|p| p.color() == color.opposite())
            .unwrap_or(false)
    }

    fn piece_handle(&mut self, sq: Square) -> &mut Option<Piece> {
        &mut self.0[sq.row][sq.col]
    }

    pub fn path_is_free(&self, start: Square, end: Square) -> bool {
        // i know it doesn't matter given how small the scale of this algorithm is
        // but i'm curious if rewriting those iterator-based .all() checks to
        // iterate over the board array directly would have cache locality benefits,
        // or if the compiler optimizes either case into something even better

        // i'm also curious if handling all 3 directions in the diagonal version
        // (which implicitly also covers both horizontal/vertical) is faster because
        // there's less branching, or if the separate paths lead to faster code because
        // it's easier for the compiler to tell what's going on for horizontal/vertical

        if mv::path_is_horizontal(start, end) {
            let first_col = cmp::min(start.col, end.col);
            let last_col = cmp::max(start.col, end.col);

            (first_col..=last_col)
                .map(|col| Square {
                    row: start.row,
                    col,
                })
                .all(|sq| sq.col == start.col || sq.col == end.col || !self.has_piece(sq))
        } else if mv::path_is_vertical(start, end) {
            let first_row = cmp::min(start.row, end.row);
            let last_row = cmp::max(start.row, end.row);

            (first_row..=last_row)
                .map(|row| Square {
                    row,
                    col: start.col,
                })
                .all(|sq| sq.row == start.row || sq.row == end.row || !self.has_piece(sq))
        } else if mv::path_is_diagonal(start, end) {
            let dr = (end.row as isize - start.row as isize).signum();
            let dc = (end.col as isize - start.col as isize).signum();

            let mut row = start.row as isize + dr;
            let mut col = start.col as isize + dc;

            while (row as usize) < end.row || (col as usize) < end.col {
                if self.has_piece(Square {
                    row: row as usize,
                    col: col as usize,
                }) {
                    return false;
                }

                row += dr;
                col += dc;
            }

            true
        } else {
            // in theory there could be better error info here
            false
        }
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
            // row of black royals,
            Self::pawns_row(Color::Black),
            // row of black pawns,
            Self::royals_row(Color::Black),
        ])
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (i, row) in self.0.iter().enumerate().rev() {
            write!(f, "{}   ", i + 1)?;

            // this rests on the fact that a vector of unit types
            // is optimized away, which i'm hoping is true
            row.iter()
                .map(|piece| match piece {
                    Some(piece) => write!(f, "{}  ", piece),
                    None => write!(f, ".  "),
                })
                .collect::<Result<Vec<_>, _>>()?;

            writeln!(f)?;
        }

        write!(f, "\n    ")?;
        ('a'..='h')
            .map(|r| write!(f, "{}  ", r))
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
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

    pub fn kind(&self) -> PieceKind {
        self.kind
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "\x1b[{}m{}\x1b[0m",
            self.color().escape_code(),
            self.kind()
        )
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
