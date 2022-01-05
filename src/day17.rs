const TARGET_AMOUNT: usize = 150;

pub fn part_a(input: &str) -> i64 {
    let mut init = [0usize; TARGET_AMOUNT + 1];
    init[0] = 1;
    let num_ways = input.lines().fold(init, |mut acc, line| {
        let size = line.parse().unwrap();
        for i in (size..=TARGET_AMOUNT).rev() {
            acc[i] += acc[i - size];
        }
        acc
    })[TARGET_AMOUNT];
    num_ways as i64
}

pub fn part_b(input: &str) -> i64 {
    let mut init = [(usize::MAX, 0usize); TARGET_AMOUNT + 1];
    init[0] = (0, 1);
    let (_, num_ways) = input.lines().fold(init, |mut acc, line| {
        let size = line.parse().unwrap();
        for i in (size..=TARGET_AMOUNT).rev() {
            if acc[i - size].0 != usize::MAX {
                if acc[i].0 > acc[i - size].0 + 1 {
                    acc[i] = (acc[i - size].0 + 1, acc[i - size].1);
                } else if acc[i].0 == acc[i - size].0 + 1 {
                    acc[i].1 += acc[i - size].1;
                }
            }
        }
        acc
    })[TARGET_AMOUNT];
    num_ways as i64
}
