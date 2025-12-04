use std::io::{self, BufRead};

fn forkliftable(map: &Vec<String>, x: usize, y: usize, w: usize, h: usize) -> bool {
    let mut c = 0;

    for i in x.saturating_sub(1).max(0)..(x+2).min(h) {
        for j in y.saturating_sub(1).max(0)..(y+2).min(w) {
            if i == x && j == y {
                continue;
            }
            if map[i].chars().nth(j).unwrap() == '@' {
                c += 1;
                if c == 4 {
                    return false;
                }
            }
        }
    }

    true
}

fn main() {
    let mut r1 = 0;
    let mut r2 = 0;

    let mut map: Vec<String> = vec![];

    for line in io::stdin().lock().lines().map(Result::unwrap) {
        map.push(line);
    }

    let w = map.first().unwrap().len();
    let h = map.len();

    for i in 0..h {
        for j in 0..w {
            if map[i].chars().nth(j).unwrap() == '@' && forkliftable(&map, i, j, w, h) {
                r1 += 1;
            }
        }
    }

    println!("{}", r1);
    println!("{}", r2);
}
