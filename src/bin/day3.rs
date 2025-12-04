use std::io::{self, BufRead};

fn get_jolt(s: &str) -> usize {
    let mut b1 = 0;
    let mut b2 = s.chars().last().unwrap() as usize - '0' as usize;

    for c in s.chars().rev().skip(1) {
        let n = c as usize - '0' as usize;
        if n >= b1 {
            b2 = b2.max(b1);
            b1 = n;
        }
    }

    return b1 * 10 + b2;
}

fn main() {
    let mut r1 = 0;
    let mut r2 = 0;

    for line in io::stdin().lock().lines().map(Result::unwrap) {
        r1 += get_jolt(&line);
    }

    println!("{}", r1);
    println!("{}", r2);
}
