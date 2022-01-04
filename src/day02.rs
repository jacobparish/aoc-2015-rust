use scan_fmt::scan_fmt;

pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (l, w, h) = scan_fmt!(line, "{d}x{d}x{d}", i64, i64, i64).unwrap();
            2 * l * w + 2 * w * h + 2 * h * l + (l * w).min(w * h).min(h * l)
        })
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (l, w, h) = scan_fmt!(line, "{d}x{d}x{d}", i64, i64, i64).unwrap();
            l * w * h + 2 * (l + w).min(w + h).min(h + l)
        })
        .sum()
}
