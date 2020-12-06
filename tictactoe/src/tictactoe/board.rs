use std::cmp::PartialEq;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Clone, Copy, Debug)]
pub enum BoardError {
    OutOfBounds,
    AlreadySet
}

impl Display for BoardError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let error = match self {
            Self::OutOfBounds => "out of bounds",
            Self::AlreadySet => "already set"
        };
        write!(f, "{}", error)
    }
}

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

#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Undecided,
    Draw,
    XWin,
    OWin,
}

#[derive(Debug)]
pub struct Board {
    data: [Tile; 9]
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, " -- -- --\n")?;
        for y in 0..3 {
            for x in 0..3 {
                write!(f, "|{}", self.at(x, y).unwrap())?
            }
            write!(f, "|\n")?;
            write!(f, " -- -- --\n")?;
        }
        FmtResult::Ok(())
    }
}

impl Board {
    pub fn new() -> Self {
        Board { data: [ Tile::EMPTY; 9 ] }
    }

    pub fn at(&self, x: usize, y: usize) -> Result<Tile, BoardError> {
        if (x >= 3) || (y >= 3) {
            Result::Err(BoardError::OutOfBounds)
        } else {
            Result::Ok(self.data[x + y * 3])
        }
    }

    pub fn state_set(&self, t: &[Tile; 3]) -> State {
        if t[0].empty() || t[1].empty() || t[2].empty() {
            State::Undecided
        } else if t[0].x() && t[1].x() && t[2].x() {
            State::XWin
        } else if t[0].o() && t[1].o() && t[2].o() {
            State::OWin
        } else {
            State::Draw
        }
    }

    pub fn state_col(&self, col: usize) -> State {
        let mut t = [Tile::EMPTY; 3];
        for i in 0..3 {
            t[i] = self.at(col, i).unwrap();
        }
        self.state_set(&t)
    }

    pub fn state_row(&self, row: usize) -> State {
        let mut t = [Tile::EMPTY; 3];
        for i in 0..3 {
            t[i] = self.at(i, row).unwrap();
        }
        self.state_set(&t)
    }

    pub fn state_diag(&self) -> State {
        let mut t = [Tile::EMPTY; 3];
        for i in 0..3 {
            t[i] = self.at(i, i).unwrap();
        }
        self.state_set(&t)
    }

    pub fn state_antidiag(&self) -> State {
        let mut t = [Tile::EMPTY; 3];
        for i in 0..3 {
            t[i] = self.at(2 - i, i).unwrap();
        }
        self.state_set(&t)
    }

    pub fn state(&self) -> State {
        let mut states = [State::Undecided; 8];
        let mut x = 0;
        for i in 0..3 {
            states[x] = self.state_row(i);
            x += 1;
        }
        for i in 0..3 {
            states[x] = self.state_col(i);
            x += 1;
        }
        states[x] = self.state_diag();
        x += 1;
        states[x] = self.state_antidiag();
        let mut result = State::Draw;
        for state in states.iter() {
            match state {
                State::XWin => return State::XWin,
                State::OWin => return State::OWin,
                State::Undecided => result = State::Undecided,
                _ => {}
            }
        }
        return result
    }

    pub fn set(&mut self, x: usize, y: usize, t: Tile) -> Result<(), BoardError> {
        if (x >= 3) || (y >= 3) {
            Result::Err(BoardError::OutOfBounds)
        } else if !self.at(x, y).unwrap().empty() {
            Result::Err(BoardError::AlreadySet)
        } else {
            self.data[x + y * 3] = t;
            Result::Ok(())
        }
    }
}
