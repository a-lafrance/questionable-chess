use crate::{board::PieceKind, err::MoveError};

pub struct Move {
    piece: PieceKind,
    start: Square,
    end: Square,
}

impl Move {
    const SIZE: usize = 5;
}

impl TryFrom<&str> for Move {
    type Error = MoveError;

    fn try_from(buf: &str) -> Result<Self, Self::Error> {
        // technically this is vulnerable to extremely large allocations
        // if the inputted line is just super long, but in practice that's
        // both unlikely and not a huge deal
        let buf: [char; Self::SIZE] = buf.chars().collect::<Vec<_>>().try_into().map_err(|_| MoveError::InvalidFormat)?;

        Ok(Move {
            piece: PieceKind::try_from(buf[0]).map_err(|_| MoveError::InvalidFormat)?,
            start: Square::try_from((buf[1], buf[2]))?,
            end: Square::try_from((buf[3], buf[4]))?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Square {
    pub row: u8,
    pub col: u8,
}

impl TryFrom<(char, char)> for Square {
    type Error = MoveError;

    fn try_from((c, r): (char, char)) -> Result<Self, Self::Error> {
        let col = if ('a'..='h').contains(&c) {
            (c as u8) - b'a'
        } else {
            return Err(MoveError::InvalidFormat);
        };

        let row = match r.to_digit(10) {
            Some(d @ 1..=8) => d as u8,
            _ => return Err(MoveError::InvalidFormat),
        };

        Ok(Square { row, col })
    }
}
