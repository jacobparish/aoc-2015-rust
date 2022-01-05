use itertools::Itertools;

fn next_pw_candidate(mut pw: Vec<u8>) -> Vec<u8> {
    let len = pw.len();
    pw[len - 1] += 1;
    for i in (0..len).rev() {
        pw[i] += match pw[i] {
            b'i' | b'n' | b'k' => 2,
            _ => 1,
        };
        if pw[i] > b'z' {
            pw[i] = b'a';
        } else {
            break;
        }
    }
    pw
}

fn is_secure_pw(pw: &Vec<u8>) -> bool {
    return pw
        .iter()
        .tuple_windows()
        .any(|(&b1, &b2, &b3)| b1 + 1 == b2 && b2 + 1 == b3)
        && pw.iter().tuple_windows().enumerate().any(|(i, (b1, b2))| {
            b1 == b2 && pw[i + 2..].iter().tuple_windows().any(|(c1, c2)| c1 == c2)
        });
}

fn next_secure_pw(mut pw: Vec<u8>) -> Vec<u8> {
    pw = next_pw_candidate(pw);
    while !is_secure_pw(&pw) {
        pw = next_pw_candidate(pw);
    }
    pw
}

pub fn part_a(input: &str) -> i64 {
    let mut pw: Vec<u8> = input.trim().as_bytes().to_vec();
    pw = next_secure_pw(pw);
    println!("{:?}", String::from_utf8(pw).unwrap());

    // This one has a non-numeric answer
    0
}

pub fn part_b(input: &str) -> i64 {
    let mut pw: Vec<u8> = input.trim().as_bytes().to_vec();
    pw = next_secure_pw(pw);
    pw = next_secure_pw(pw);
    println!("{:?}", String::from_utf8(pw).unwrap());

    // This one has a non-numeric answer
    0
}
