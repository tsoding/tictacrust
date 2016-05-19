
// [1] [2] [3]
// [4]  x  [6]
// [7] [8] [9]
// > 5

enum Cell {
    Empty,
    Cross,
    Zero
}

fn print_cell(cell: &Cell, idx: u32) {
    match *cell {
        Cell::Empty => print!("[{}]", idx),
        Cell::Cross => print!(" x "),
        Cell::Zero => print!(" o "),
    }
}

fn print_board(board: &[Cell; 9]) {
    let mut idx = 0;
    for row in board.chunks(3) {
        for cell in row {
            print_cell(cell, idx);
            idx += 1
        }
        println!("")
    }
}

fn main() {
    let board: [Cell; 9] = [Cell::Empty, Cell::Empty, Cell::Empty,
                            Cell::Empty, Cell::Cross, Cell::Empty,
                            Cell::Empty, Cell::Empty, Cell::Empty];

    print_board(&board);
}
