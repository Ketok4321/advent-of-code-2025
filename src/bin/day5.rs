use std::io::{self, Read};

fn main() {
    let mut r1 = 0;
    let mut r2 = 0;

    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (ranges_s, ing_s) = input.split_once("\n\n").unwrap();

    let mut ranges = vec![];
    let mut ing = vec![];

    for r in ranges_s.split_whitespace() {
        let (min, max) = r.split_once("-").unwrap();
        ranges.push((min.parse::<u64>().unwrap(), max.parse::<u64>().unwrap()));
    }

    for i in ing_s.split_whitespace() {
        ing.push(i.parse::<u64>().unwrap());
    }

    for i in ing {
        for (min, max) in &ranges {
            if i >= *min && i <= *max {
                r1 += 1;
                break;
            }
        }
    }

    let mut ranges_ord = ranges.clone();
    ranges_ord.sort_by(|(m1, _), (m2, _)| m1.cmp(m2));
    let mut up_to = 0;

    for (min, max) in ranges_ord {
        if max >= up_to {
            r2 += max - min.max(up_to) + 1;
            up_to = max + 1;
        }
    }

    println!("{}", r1);
    println!("{}", r2);
}
