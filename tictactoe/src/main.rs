use std::io::stdin;
use rand::random;

mod tictactoe;
use tictactoe::ai_move;
use tictactoe::Board;
use tictactoe::State;
use tictactoe::Tile;

fn main() {
    let player_tile = if random() { Tile::X } else { Tile::O };
    let ai_tile = player_tile.opposite();
    let mut board = Board::new();
    if player_tile.o() {
        board = ai_move(&board, ai_tile).unwrap();
    }
    println!("{}", board);
    loop {
        println!("{}: ", &player_tile);
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.find(" ") {
            Some(i) => {
                let x: usize = match input[..i].parse() {
                    Ok(d) => d,
                    Err(e) => {
                        println!("invalid x coord: {}", e);
                        continue;
                    }
                };
                let y: usize = match input[i+1..].trim().parse() {
                    Ok(d) => d,
                    Err(e) => {
                        println!("invalid y coord: {}", e);
                        continue;
                    }
                };
                if board.set(x, y, player_tile).is_err() {
                    println!("invalid coords");
                    continue;
                }
            }
            None => {
                println!("Argle-bargle, glop-glyf!?!");
                continue;
            }
        }
        println!("{}", board);
        match ai_move(&board, ai_tile) {
            Some(new_board) => board = new_board,
            None => {}
        }
        println!("{}", board);
        match board.state() {
            State::Undecided => {
                continue;
            }
            State::Draw => {
                println!("it's a draw!");
                break;
            }
            State::XWin => {
                println!("❌ wins");
                break;
            }
            State::OWin => {
                println!("⭕ wins");
                break;
            }
        }
    }
}
