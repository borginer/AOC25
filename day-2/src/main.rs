use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("couldn't read input");

    let mut count1 = 0;
    let mut count2 = 0;

    for pair in input.split(",") {
        let parts: Vec<i64> = pair
            .split("-")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();

        let from = parts[0];
        let to = parts[1];

        count1 += sum_invalid_numbers(from, to, is_invalid_number_part1);
        count2 += sum_invalid_numbers(from, to, is_invalid_number_part2);
    }

    println!("Part 1 Result: {}", count1);
    println!("Part 2 Result: {}", count2);
}

fn sum_invalid_numbers(from: i64, to: i64, is_invalid_number: fn(i64) -> bool) -> i64 {
    let mut sum = 0;

    for num in from..=to {
        if is_invalid_number(num) {
            sum += num;
        }
    }

    sum
}

fn is_invalid_number_part1(num: i64) -> bool {
    let num: Vec<char> = num.to_string().chars().collect();

    if num.len() % 2 != 0 {
        return false;
    }

    for i in 0..num.len() / 2 {
        if num[i] != num[i + num.len() / 2] {
            return false;
        }
    }

    true
}

fn is_invalid_number_part2(num: i64) -> bool {
    let chars: Vec<char> = num.to_string().chars().collect();
    let chars_len = chars.len();

    for n_parts in 2..=chars_len {
        // can be divided into @n_parts parts
        if chars_len == (chars_len / n_parts) * n_parts {
            let part_len = chars_len / n_parts;
            let mut is_invalid = true;

            'outer: for i in 0..part_len {
                for part in 0..n_parts {
                    if chars[i] != chars[i + part * part_len] {
                        is_invalid = false;
                        break 'outer;
                    }
                }
            }

            if is_invalid {
                return is_invalid;
            }
        }
    }

    return false;
}
