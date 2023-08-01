use crate::{err::MoveError, game::PieceKind};

pub struct Move {
    piece: PieceKind,
    start: Square,
    end: Square,
}

impl TryFrom<[u8; 5]> for Move {
    type Error = MoveError;

    fn try_from(buf: [u8; 5]) -> Result<Self, Self::Error> {
        Ok(Move {
            piece: PieceKind::try_from(buf[0] as char).map_err(|_| MoveError::InvalidFormat)?,
            start: Square::try_from((buf[1] as char, buf[2] as char))?,
            end: Square::try_from((buf[3] as char, buf[4] as char))?,
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
