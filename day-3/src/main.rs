use std::io;

fn main() {
    let mut total_max_joltage_p1 = 0;
    let mut total_max_joltage_p2 = 0;

    for line in io::stdin().lines() {
        let line = line.unwrap();
        total_max_joltage_p1 += max_line_joltage(&line, 2);
        total_max_joltage_p2 += max_line_joltage(&line, 12);
    }

    println!("Total Max Joltage Part 1: {}", total_max_joltage_p1);
    println!("Total Max Joltage Part 2: {}", total_max_joltage_p2);
}

fn max_line_joltage(line: &String, digits: usize) -> usize {
    let line: Vec<char> = line.chars().collect();
    let mut max_joltage = 0;
    let mut start_index = 0;

    for i in (0..digits).rev() {
        // search upto (line.len - i) because we will need at least i more digits to form the full number
        let max_index = max_index(&line, start_index..line.len() - i);
        max_joltage += char_to_digit(line[max_index]) * 10_usize.pow(i as u32);
        start_index = max_index + 1;
    }

    return max_joltage;
}

fn max_index(line: &Vec<char>, range: std::ops::Range<usize>) -> usize {
    let mut max_left_idx = range.start;
    let mut max_value = char_to_digit(line[range.start]);

    for i in range {
        let digit = char_to_digit(line[i]);
        if digit > max_value {
            max_value = digit;
            max_left_idx = i;
        }
    }

    return max_left_idx;
}

fn char_to_digit(c: char) -> usize {
    (c as u8 - b'0') as usize
}
