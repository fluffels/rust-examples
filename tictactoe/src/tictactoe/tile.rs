use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    EMPTY,
    X,
    O
}

impl Tile {
    pub fn empty(&self) -> bool {
        *self == Tile::EMPTY
    }

    pub fn x(&self) -> bool {
        *self == Tile::X
    }
    
    pub fn o(&self) -> bool {
        *self == Tile::O
    }

    pub fn opposite(&self) -> Self {
        match self {
            Tile::O => Tile::X,
            Tile::X => Tile::O,
            Tile::EMPTY => Tile::EMPTY
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let glyph = match self {
            Self::EMPTY => "　",
            Self::X => "❌",
            Self::O => "⭕",
        };
        write!(f, "{}", glyph)
    }
}
