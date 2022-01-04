use scan_fmt::scan_fmt;

pub fn part_a(input: &str) -> i64 {
    input.lines().map(|line| {
        let (on_off_str, x1, y1, x2, y2) = scan_fmt!(
            line,
            "{} {d},{d} through {d},{d}",
            String,
            usize,
            usize,
            usize,
            usize
        )
        .unwrap();
    });

    0
}

pub fn part_b(_input: &str) -> i64 {
    0
}
