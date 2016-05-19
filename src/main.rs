
// [1] [2] [3]
// [4]  x  [6]
// [7] [8] [9]
// > 5

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Cross,
    Zero
}

#[derive(Copy, Clone)]
enum State {
    CrossTurn,
    ZeroTurn,
    GameOver
}

fn read_index() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.parse().unwrap()
}

fn print_cell(cell: &Cell, idx: usize) {
    match *cell {
        Cell::Empty => print!("[{}]", idx),
        Cell::Cross => print!(" x "),
        Cell::Zero => print!(" o "),
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

fn main() {
    let board: [Cell; 9] = [Cell::Empty; 9];

    print_board(&board);
}
