use crate::{board::PieceKind, err::MoveError};

pub struct Move {
    piece: PieceKind,
    start: Square,
    end: Square,
}

impl Move {
    const SIZE: usize = 5;

    pub fn piece(&self) -> PieceKind {
        self.piece
    }

    pub fn start(&self) -> Square {
        self.start
    }

    pub fn end(&self) -> Square {
        self.end
    }
}

impl TryFrom<&str> for Move {
    type Error = MoveError;

    fn try_from(buf: &str) -> Result<Self, Self::Error> {
        // technically this is vulnerable to extremely large allocations
        // if the inputted line is just super long, but in practice that's
        // both unlikely and not a huge deal
        let buf: [char; Self::SIZE] = buf
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| MoveError::InvalidFormat)?;

        Ok(Move {
            piece: PieceKind::try_from(buf[0]).map_err(|_| MoveError::InvalidFormat)?,
            start: Square::try_from((buf[1], buf[2]))?,
            end: Square::try_from((buf[3], buf[4]))?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Square {
    pub row: usize,
    pub col: usize,
}

impl TryFrom<(char, char)> for Square {
    type Error = MoveError;

    fn try_from((c, r): (char, char)) -> Result<Self, Self::Error> {
        let col = if ('a'..='h').contains(&c) {
            (c as usize) - ('a' as usize)
        } else {
            return Err(MoveError::InvalidFormat);
        };

        let row = match r.to_digit(10) {
            Some(d @ 1..=8) => d as usize - 1,
            _ => return Err(MoveError::InvalidFormat),
        };

        Ok(Square { row, col })
    }
}

pub fn path_is_horizontal(start: Square, end: Square) -> bool {
    start.row == end.row
}

pub fn horizontal_displacement(start: Square, end: Square) -> usize {
    (start.col as isize - end.col as isize).abs() as usize
}

pub fn path_is_vertical(start: Square, end: Square) -> bool {
    start.col == end.col
}

pub fn vertical_displacement(start: Square, end: Square) -> usize {
    (start.row as isize - end.row as isize).abs() as usize
}

pub fn path_is_diagonal(start: Square, end: Square) -> bool {
    let dr = (start.row as isize - end.row as isize).abs();
    let dc = (start.col as isize - end.col as isize).abs();

    dr == dc
}

// A straight path is one that goes in exactly one direction,
// horizontal, vertical, or diagonal.
pub fn path_is_straight(start: Square, end: Square) -> bool {
    path_is_horizontal(start, end) || path_is_vertical(start, end) || path_is_diagonal(start, end)
}
