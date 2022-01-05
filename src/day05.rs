use itertools::Itertools;

fn is_vowel(b: u8) -> bool {
    b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u'
}

fn is_nice_pair(pair: (u8, u8)) -> bool {
    pair != (b'a', b'b') && pair != (b'c', b'd') && pair != (b'p', b'q') && pair != (b'x', b'y')
}

fn is_nice_a(s: &str) -> bool {
    s.bytes().filter(|&b| is_vowel(b)).count() >= 3
        && s.bytes().tuple_windows().any(|(b1, b2)| b1 == b2)
        && s.bytes().tuple_windows().all(|pair| is_nice_pair(pair))
}

pub fn part_a(input: &str) -> i64 {
    input.lines().filter(|line| is_nice_a(line)).count() as i64
}

fn is_nice_b(s: &str) -> bool {
    s.bytes().tuple_windows().any(|(b1, _, b2)| b1 == b2)
        && s.bytes()
            .tuple_windows()
            .enumerate()
            .any(|(i, p1): (usize, (u8, u8))| {
                s[i + 2..]
                    .bytes()
                    .tuple_windows()
                    .any(|p2: (u8, u8)| p2 == p1)
            })
}

pub fn part_b(input: &str) -> i64 {
    input.lines().filter(|line| is_nice_b(line)).count() as i64
}
