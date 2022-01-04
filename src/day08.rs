pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let mut ndiff = 2;
            let mut i = 1;
            while i < bytes.len() - 1 {
                match (bytes[i], bytes[i + 1]) {
                    (b'\\', b'x') => {
                        ndiff += 3;
                        i += 4;
                    }
                    (b'\\', _) => {
                        ndiff += 1;
                        i += 2;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            ndiff
        })
        .sum::<usize>() as i64
}

pub fn part_b(input: &str) -> i64 {
    input
        .lines()
        .map(|line| 2 + line.bytes().filter(|&b| b == b'"' || b == b'\\').count())
        .sum::<usize>() as i64
}
