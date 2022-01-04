use scan_fmt::scan_fmt;

use crate::utils::pow_mod;

pub fn part_a(input: &str) -> i64 {
    let (row, col) = scan_fmt!(input, "To continue, please consult the code grid in the manual.  Enter the code at row {d}, column {d}", usize, usize).unwrap();
    let diag = row + col - 1;
    let code_num = diag * (diag - 1) / 2 + col;
    let init_value = 20151125;
    let base = 252533;
    let modulus = 33554393;
    (init_value * pow_mod(base, code_num - 1, modulus)) % modulus
}

pub fn part_b(_input: &str) -> i64 {
    0
}
