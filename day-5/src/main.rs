use std::{
    io::{self, Read},
    num::ParseIntError,
};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("should be able to read from stdin");

    println!("Part 1 Result: {}", part1(&input).expect("no errors :)"));
    println!("Part 2 Result: {}", part2(&input).expect("no errors :)"));
}

fn part1(input: &String) -> Result<u64, ParseIntError> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut first_part = true;
    let mut count = 0;

    for line in input.lines() {
        if line == "" {
            first_part = false;
            continue;
        }
        if first_part {
            let mut parts = line.split("-");
            let p1: u64 = parts.next().unwrap().parse()?;
            let p2: u64 = parts.next().unwrap().parse()?;
            ranges.push((p1, p2));
        } else {
            let num: u64 = line.parse()?;
            for range in ranges.iter() {
                if inside(num, range) {
                    count += 1;
                    break;
                }
            }
        }
    }

    Ok(count)
}

fn part2(input: &String) -> Result<u64, ParseIntError> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        if line == "" {
            break;
        }

        let mut parts = line.split("-");
        let p1: u64 = parts.next().unwrap().parse()?;
        let p2: u64 = parts.next().unwrap().parse()?;

        let mut range = (p1, p2);

        let mut new_ranges: Vec<(u64, u64)> = Vec::new();

        for irange in ranges.iter() {
            if colide(&range, irange) {
                range = merge(&range, irange);
            } else {
                new_ranges.push(irange.clone());
            }
        }

        new_ranges.push(range);
        ranges = new_ranges;
    }

    let mut sum = 0;
    for range in ranges.iter() {
        sum += range.1 - range.0 + 1;
    }

    Ok(sum)
}

fn colide(a: &(u64, u64), b: &(u64, u64)) -> bool {
    inside(a.0, b) || inside(a.1, b) || inside(b.0, a) || inside(b.1, a)
}

fn inside(num: u64, range: &(u64, u64)) -> bool {
    num >= range.0 && num <= range.1
}

fn merge(a: &(u64, u64), b: &(u64, u64)) -> (u64, u64) {
    (a.0.min(b.0), a.1.max(b.1))
}
