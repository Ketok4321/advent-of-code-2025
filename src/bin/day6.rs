use std::io::{self, BufRead};

const N: usize = 1000; // hardcoded :<

fn main() {
    let mut r1 = 0;
    let mut r2 = 0;

    let mut seq = vec![vec![]; N];
    let mut op = vec!['\0'; N];

    for line in io::stdin().lock().lines().map(Result::unwrap) {
        for (i, n) in line.split_whitespace().enumerate() {
            if let Ok(r) = n.parse::<usize>() {
                seq[i].push(r);
            } else {
                op[i] = n.chars().nth(0).unwrap();
            }
        }
    }

    for i in 0..N {
        r1 += match op[i] {
            '+' => seq[i].iter().sum(),
            '*' => seq[i].iter().fold(1, |acc, n| acc * n),
            _ => panic!(),
        }
    }

    println!("{}", r1);
    println!("{}", r2);
}
