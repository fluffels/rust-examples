use super::Board;
use super::State;
use super::Tile;

pub fn ai_move(board: &Board, turn: Tile) -> Option<Board> {
    let mut node = Node::new(&board, turn);
    node.evaluate(turn);
    let move_node = node.best_move()?;
    Some(*move_node.board())
}

#[derive(Debug)]
pub struct Node {
    board: Board,
    turn: Tile,
    value: i64,
    children: Vec<Node>,
}

impl Node {
    pub fn new(board: &Board, turn: Tile) -> Node {
        Node { board: *board, turn, children: Vec::new(), value: 0 }
    }

    pub fn board(&self) -> &Board { &self.board }
    pub fn value(&self) -> i64 { self.value }

    pub fn evaluate(&mut self, tile: Tile) -> i64{
        match self.board.state() {
            State::OWin => {
                self.value = if tile.o() { 1 } else { -1 }
            }
            State::XWin => {
                self.value = if tile.x() { 1 } else { -1 }
            }
            State::Draw => {
                self.value = 0
            }
            State::Undecided => {
                for y in 0..3 {
                    for x in 0..3 {
                        let mut board = self.board;
                        match board.set(x, y, self.turn) {
                            Ok(()) => {
                                let mut child = Node::new(&board, self.turn.opposite());
                                child.evaluate(tile);
                                self.children.push(child);
                            },
                            Err(_) => continue
                        }
                    }
                }
                for child in &self.children {
                    if tile == self.turn {
                        if child.value() > self.value {
                            self.value = child.value()
                        }
                    } else {
                        if child.value() < self.value {
                            self.value = child.value()
                        }
                    }
                }
            }
        }
        self.value
    }

    pub fn best_move(&self) -> Option<&Node> {
        let mut result = self.children.first()?;
        for child in &self.children {
            if child.value > result.value {
                result = child;
            }
        }
        Some(result)
    }
}
