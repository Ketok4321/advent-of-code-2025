use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let mut r1 = 0;

    let mut pos = HashMap::new();

    for line in io::stdin().lock().lines().map(Result::unwrap) {
        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                pos.insert(i, 1);
            } else if c == '^' {
                if let Some((_, n)) = pos.remove_entry(&i) {
                    *pos.entry(i - 1).or_insert(0) += n;
                    *pos.entry(i + 1).or_insert(0) += n;
                    r1 += 1;
                }
            }
        }
    }

    println!("{}", r1);
    println!("{}", pos.values().sum::<usize>());
}
