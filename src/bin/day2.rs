use std::io::{self, BufRead};

fn prime_fac(n: usize) -> Vec<usize> {
    let mut n = n;
    let mut i = 2;
    let mut r = vec![];

    while n != 1 {
        if n % i == 0 {
            r.push(i);
            while n % i == 0 {
                n /= i;
            }
        }
        i += 1;
    }

    r
}

fn check1(s: &str) -> bool {
    if s.len() % 2 == 0 {
        s[0..s.len()/2] == s[s.len()/2..]
    } else {
        false
    }
}

fn check2(s: &str, l: usize) -> bool {
    for i in l..s.len() {
        if s.chars().nth(i) != s.chars().nth(i - l) {
            return false;
        }
    }

    true
}

fn in_range(start: usize, end: usize) -> (usize, usize) {
    let mut r1 = 0;
    let mut r2 = 0;
    
    for n in start..end+1 {
        let s = n.to_string();
        if check1(&s) {
            r1 += n;
        }
        for p in prime_fac(s.len()) {
            if check2(&s, s.len() / p) {
                r2 += n;
                break;
            }
        }
    }
    (r1, r2)
}

fn main() {
    let mut r1 = 0;
    let mut r2 = 0;

    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).expect("Failed to read line");

    let ranges = line.split(',');
    for r in ranges {
        let (min, max) = r.split_once('-').unwrap();
        let (d1, d2) = in_range(min.trim().parse::<usize>().unwrap(), max.trim().parse::<usize>().unwrap());
        r1 += d1;
        r2 += d2;
    }

    println!("{}", r1);
    println!("{}", r2);
}
