#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Undecided,
    Draw,
    XWin,
    OWin,
}
