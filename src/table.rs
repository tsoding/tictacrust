pub fn check_row<F>(n: usize, mut predicate: F) -> bool where F: FnMut(usize) -> bool {
    let mut streak = true;
    for j in 0..n {
        streak &= predicate(j);
    }
    streak
}

pub fn check_table<F>(n: usize, m: usize, mut predicate: F) -> bool where F: FnMut(usize, usize) -> bool {
    for i in 0..n {
        if check_row(m, |j| predicate(i, j)) {
            return true
        }
    }
    false
}
