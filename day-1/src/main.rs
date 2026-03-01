use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("couldn't read stdin");

    println!("Part 1: count = {}", calc_part(&input, part1_rotate));
    println!("Part 2: count = {}", calc_part(&input, part2_rotate));
}

struct RotateResult {
    count: i32,
    dial: i32,
}

fn calc_part(input: &String, rotate: fn(i32, &str, i32) -> RotateResult) -> i32 {
    let mut dial = 50;
    let mut count = 0;

    for line in input.lines() {
        let line = line;
        let dir = line.get(0..1);
        let num = line.get(1..);

        if let (Some(dir), Some(num)) = (dir, num) {
            let num: i32 = num.parse().expect("can't parse number");
            let rotate_result = rotate(dial, dir, num);
            count += rotate_result.count;
            dial = rotate_result.dial;
        }
    }

    return count;
}

fn part1_rotate(mut dial: i32, dir: &str, by: i32) -> RotateResult {
    let mut count = 0;
    if dir == "R" {
        dial += by;
    } else if dir == "L" {
        dial -= by;
    }

    if dial % 100 == 0 {
        count += 1;
    }

    RotateResult { count, dial }
}

fn part2_rotate(mut dial: i32, dir: &str, by: i32) -> RotateResult {
    let mut count = 0;
    for _ in 0..by {
        if dir == "R" {
            dial += 1;
        } else if dir == "L" {
            dial -= 1;
        }

        if dial % 100 == 0 {
            count += 1;
        }
    }

    RotateResult { count, dial }
}
