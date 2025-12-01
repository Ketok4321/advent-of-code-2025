use std::io::{self, BufRead};

const N: isize = 100;

fn main() {
    let mut x = 50;
    let mut r1 = 0;
    let mut r2 = 0;

    for line in io::stdin().lock().lines().map(Result::unwrap) {
        let d = line.chars().nth(0).unwrap();
        let n = line[1..].parse::<isize>().unwrap();

        match d {
            'L' => {
                if x == 0 {
                    r2 += n / N;
                } else {
                    r2 += (N + n - x) / N;
                }
                x = (N + x - (n % N)) % N;
            },
            'R' => {
                r2 += (x + n) / N;
                x = (x + (n % N)) % N;
            },
            _ => panic!(),
        }
        if x == 0 {
            r1 += 1;
        }
    }

    println!("{}", r1);
    println!("{}", r2);
}
