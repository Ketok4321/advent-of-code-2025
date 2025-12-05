use std::io::{self, BufRead};

fn forkliftable(map: &Vec<Vec<char>>, x: usize, y: usize, w: usize, h: usize) -> bool {
    let mut c = 0;

    for i in x.saturating_sub(1).max(0)..(x+2).min(h) {
        for j in y.saturating_sub(1).max(0)..(y+2).min(w) {
            if i == x && j == y {
                continue;
            }
            if map[i][j] == '@' {
                c += 1;
                if c == 4 {
                    return false;
                }
            }
        }
    }

    true
}

fn remove(map: &mut Vec<Vec<char>>, w: usize, h: usize) -> usize {
    let mut dr = 0;
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '@' && forkliftable(&map, i, j, w, h) {
                map[i][j] = '.';
                dr += 1;
            }
        }
    }
    dr
}

fn main() {
    let mut r1 = 0;
    let mut r2 = 0;

    let mut map: Vec<Vec<char>> = vec![];

    for line in io::stdin().lock().lines().map(Result::unwrap) {
        map.push(line.chars().collect());
    }

    let w = map.first().unwrap().len();
    let h = map.len();

    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '@' && forkliftable(&map, i, j, w, h) {
                r1 += 1;
            }
        }
    }

    loop {
        let dr = remove(&mut map, w, h);
        r2 += dr;
        if dr == 0 {
            break
        }
    }

    println!("{}", r1);
    println!("{}", r2);
}
