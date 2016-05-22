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
    PlayerTurn(Player),
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
        Figure(player) => print!(" {} ", player)
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

fn opposite_player(player: Player) -> Player {
    match player {
        Cross => Zero,
        Zero => Cross
    }
}

fn player_turn(board: &mut [Cell; 9],
               player: Player) -> State {
    print_board(board);
    println!("{}> ", player);
    let index = read_index();

    match board[index - 1] {
        Cell::Empty => {
            board[index - 1] = Figure(player);
            PlayerTurn(opposite_player(player))
        },

        _ => {
            println!("The cell is not empty!");
            PlayerTurn(player)
        }
    }
}

fn main() {
    let mut board = [Empty; 9];
    let mut state = PlayerTurn(Cross);

    while let PlayerTurn(player) = state {
        state = player_turn(&mut board, player)
    }
}
