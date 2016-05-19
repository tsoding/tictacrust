
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

fn print_cell(cell: &Cell, idx: usize) {
    match *cell {
        Cell::Empty => print!("[{}]", idx),
        Cell::Cross => print!(" x "),
        Cell::Zero => print!(" o "),
    }
}

fn print_board(board: &[Cell; 9]) {
    for (idx, cell) in board.iter().enumerate() {
        print_cell(cell, idx);
    }
}

fn main() {
    let board: [Cell; 9] = [Cell::Empty; 9];

    print_board(&board);
}
