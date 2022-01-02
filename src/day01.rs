use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    input[..input.len() - 1].bytes().fold(0, |acc, b| match b {
        b'(' => acc + 1,
        b')' => acc - 1,
        _ => panic!("unexpected character"),
    })
}

pub fn part_b(input: &str) -> i64 {
    input[..input.len() - 1]
        .bytes()
        .enumerate()
        .fold_while(0, |acc, (i, b)| match b {
            b'(' => Continue(acc + 1),
            b')' => {
                if acc == 0 {
                    Done(i + 1)
                } else {
                    Continue(acc - 1)
                }
            }
            _ => panic!("unexpected character"),
        })
        .into_inner() as i64
}
