// [1] [2] [3]
// [4]  x  [6]
// [7] [8] [9]
// > 5

use std::fmt;

#[derive(Copy, Clone)]
enum Player {
    Cross,
    Zero
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cross => write!(f, "x"),
            Zero => write!(f, "o")
        }
    }
}

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Figure(Player)
}

#[derive(Copy, Clone)]
enum State {
    CrossTurn,
    ZeroTurn,
    GameOver
}

use Player::*;
use Cell::*;
use State::*;

fn read_index() -> usize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn print_cell(cell: &Cell, idx: usize) {
    match *cell {
        Empty => print!("[{}]", idx),
        Figure(Cross) => print!(" x "),
        Figure(Zero) => print!(" o "),
    }
}

fn print_board(board: &[Cell; 9]) {
    for (i, row) in board.chunks(3).enumerate() {
        for (j, cell) in row.iter().enumerate() {
            print_cell(cell, i * 3 + j + 1)
        }
        println!("")
    }
}

fn player_turn(board: &mut [Cell; 9],
               prompt: &str,
               cell: Cell,
               current_state: State,
               next_state: State) -> State {
    print_board(board);
    println!("{}> ", prompt);
    let index = read_index();

    match board[index - 1] {
        Cell::Empty => {
            println!("The cell is not empty!");
            current_state
        },

        _ => {
            board[index - 1] = cell;
            next_state
        }
    }
}

fn main() {
    let mut board = [Cell::Empty; 9];
    let mut state = State::CrossTurn;

    loop {
        match state {
            CrossTurn => {
                state = player_turn(&mut board, "x", Figure(Cross), state, ZeroTurn)
            },

            ZeroTurn => {
                state = player_turn(&mut board, "o", Figure(Zero), state, CrossTurn)
            },

            GameOver => {
                unimplemented!()
            }
        }
    }
}
