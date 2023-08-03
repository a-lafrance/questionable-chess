use std::{
    cmp,
    fmt::{self, Display, Formatter},
};

use crate::{
    board::{Board, Piece, PieceKind},
    err::MoveError,
    mv::{self, Move},
};

#[derive(Default)]
pub struct Game {
    board: Board,
    current_player: Color,
}

impl Game {
    pub fn current_player(&self) -> Color {
        self.current_player
    }

    fn change_turn(&mut self) {
        self.current_player = self.current_player.opposite();
    }

    pub fn make_move(&mut self, mv: Move) -> Result<TurnOutcome, MoveError> {
        if !self.is_valid_path(&mv) {
            return Err(MoveError::InvalidPath);
        }

        self.board
            .move_piece(mv.piece(), self.current_player, mv.start(), mv.end())
            .map(|taken| {
                self.change_turn();

                match taken {
                    Some(piece) => match piece.kind() {
                        PieceKind::King => TurnOutcome::Win(self.current_player),
                        _ => TurnOutcome::Taken(piece),
                    },

                    None => TurnOutcome::Continue,
                }
            })
    }

    fn is_valid_path(&self, mv: &Move) -> bool {
        if mv.start() == mv.end() {
            // the piece can't stay in place
            return false;
        }

        // make sure end square doesn't already have friendly piece
        let end_has_friendly_piece = self
            .board
            .piece(mv.end())
            .map(|piece| piece.color() == self.current_player)
            .unwrap_or(false);

        if end_has_friendly_piece {
            return false;
        }

        match mv.piece() {
            PieceKind::Rook => {
                // make sure path is either horizontal or vertical
                // make sure path is not blocked
                self.board.path_is_free(mv.start(), mv.end())
                    && (mv::path_is_horizontal(mv.start(), mv.end())
                        || mv::path_is_vertical(mv.start(), mv.end()))
            },

            PieceKind::Bishop => {
                // make sure path is diagonal
                // make sure path not blocked
                self.board.path_is_free(mv.start(), mv.end())
                    && mv::path_is_diagonal(mv.start(), mv.end())
            },

            PieceKind::Queen => {
                // make sure path is horizontal or vertical or diagonal
                // make sure path not blocked
                self.board.path_is_free(mv.start(), mv.end())
                    && mv::path_is_straight(mv.start(), mv.end())
            },

            PieceKind::King => {
                // regardless of direction, make sure path is 1 square away
                let displacement = cmp::max(
                    mv::horizontal_displacement(mv.start(), mv.end()),
                    mv::vertical_displacement(mv.start(), mv.end()),
                );

                displacement == 1
            },

            PieceKind::Knight => {
                // make sure path is either:
                // 2 horizontal, 1 vertical displacement
                // 2 vertical, 1 horizontal displacement
                let h = mv::horizontal_displacement(mv.start(), mv.end());
                let v = mv::vertical_displacement(mv.start(), mv.end());

                (h == 2 && v == 1) || (h == 1 && v == 2)
            },

            PieceKind::Pawn => {
                let start_row = match self.current_player() {
                    Color::White => Board::WHITE_PAWN_START_ROW,
                    Color::Black => Board::BLACK_PAWN_START_ROW,
                };

                let v = mv::vertical_displacement(mv.start(), mv.end());
                let can_move_forward = mv::path_is_vertical(mv.start(), mv.end()) && v == 1
                    || (v == 2 && mv.start().row == start_row) && !self.board.has_piece(mv.end());

                let can_move_diagonal = mv::path_is_diagonal(mv.start(), mv.end())
                    && v == 1
                    && self
                        .board
                        .has_opposing_piece(mv.end(), self.current_player());

                let valid_direction = match self.current_player() {
                    Color::White => mv.start().row < mv.end().row,
                    Color::Black => mv.start().row > mv.end().row,
                };

                (can_move_forward || can_move_diagonal) && valid_direction
            },
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TurnOutcome {
    Continue,
    Taken(Piece),
    Win(Color),
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

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn escape_code(&self) -> u8 {
        match self {
            Color::White => 31,
            Color::Black => 34,
        }
    }
}
