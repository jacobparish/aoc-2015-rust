use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point(i64, i64);

impl Point {
    fn move_in_dir(self, dir: u8) -> Point {
        let Point(x, y) = self;
        match dir {
            b'<' => Point(x - 1, y),
            b'>' => Point(x + 1, y),
            b'^' => Point(x, y - 1),
            b'v' => Point(x, y + 1),
            _ => panic!("unexpected character"),
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let (_, visited) = input.trim().bytes().fold(
        (Point(0, 0), HashSet::from([Point(0, 0)])),
        |(curr, mut visited), dir| {
            let next = curr.move_in_dir(dir);
            visited.insert(next);
            (next, visited)
        },
    );
    visited.len() as i64
}

pub fn part_b(input: &str) -> i64 {
    let (_, _, visited) = input.trim().bytes().fold(
        (Point(0, 0), Point(0, 0), HashSet::from([Point(0, 0)])),
        |(curr1, curr2, mut visited), dir| {
            let next = curr1.move_in_dir(dir);
            visited.insert(next);
            (curr2, next, visited)
        },
    );
    visited.len() as i64
}
