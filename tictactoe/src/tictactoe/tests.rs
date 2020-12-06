#![cfg(test)]

use super::Board;
use super::ai::ai_move;
use super::State;
use super::Tile;

#[test]
fn test_blank() {
    let board = Board::new();
    for x in 0..2 {
        for y in 0..2 {
            assert!(board.at(x, y).unwrap() == Tile::EMPTY);
        }
    }
    assert!(board.state() == State::Undecided);
}

#[test]
fn test_col_win() {
    let mut board = Board::new();
    board.set(0, 0, Tile::O).unwrap();
    board.set(0, 1, Tile::O).unwrap();
    board.set(0, 2, Tile::O).unwrap();
    assert!(board.state() == State::OWin);
}

#[test]
fn test_row_win() {
    let mut board = Board::new();
    board.set(0, 0, Tile::O).unwrap();
    board.set(1, 0, Tile::O).unwrap();
    board.set(2, 0, Tile::O).unwrap();
    assert!(board.state() == State::OWin);
}

#[test]
fn test_diag_win() {
    let mut board = Board::new();
    board.set(0, 0, Tile::X).unwrap();
    board.set(1, 1, Tile::X).unwrap();
    board.set(2, 2, Tile::X).unwrap();
    assert!(board.state() == State::XWin);
}

#[test]
fn test_antidiag_win() {
    let mut board = Board::new();
    board.set(2, 0, Tile::X).unwrap();
    board.set(1, 1, Tile::X).unwrap();
    board.set(0, 2, Tile::X).unwrap();
    assert!(board.state() == State::XWin);
}

#[test]
fn test_draw() {
    let mut board = Board::new();
    board.set(0, 0, Tile::X).unwrap();
    board.set(1, 0, Tile::X).unwrap();
    board.set(2, 0, Tile::O).unwrap();
    board.set(0, 1, Tile::O).unwrap();
    board.set(1, 1, Tile::O).unwrap();
    board.set(2, 1, Tile::X).unwrap();
    board.set(0, 2, Tile::X).unwrap();
    board.set(1, 2, Tile::X).unwrap();
    board.set(2, 2, Tile::O).unwrap();
    assert!(board.state() == State::Draw);
}

#[test]
fn test_undecided() {
    let mut board = Board::new();
    board.set(0, 0, Tile::X).unwrap();
    board.set(1, 0, Tile::X).unwrap();
    board.set(2, 0, Tile::O).unwrap();
    board.set(0, 1, Tile::O).unwrap();
    board.set(1, 1, Tile::O).unwrap();
    board.set(2, 1, Tile::X).unwrap();
    board.set(0, 2, Tile::X).unwrap();
    board.set(1, 2, Tile::X).unwrap();
    assert!(board.state() == State::Undecided);
}

#[test]
fn test_ai() {
    let mut board = Board::new();
    board.set(1, 1, Tile::X).ok();
    ai_move(&board, Tile::O).unwrap();
}
