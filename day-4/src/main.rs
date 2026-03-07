use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("able to read stdin");

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut vec_line: Vec<char> = Vec::new();
        let mut line: Vec<char> = line.chars().collect();

        vec_line.push('.');
        vec_line.append(&mut line);
        vec_line.push('.');

        grid.push(vec_line);
    }

    let cols = grid[0].len();
    grid.insert(0, vec!['.'; cols]);
    grid.push(vec!['.'; cols]);

    println!("Part 1: {}", part1_paper(&grid));
    println!("Part 2: {}", part2_paper(&mut grid));
}

fn part1_paper(grid: &Vec<Vec<char>>) -> i32 {
    let mut paper_rolls = 0;

    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() {
            if grid[i][j] == '@' && count_adjacent(&grid, i, j) < 4 {
                paper_rolls += 1;
            }
        }
    }

    paper_rolls
}

fn part2_paper(grid: &mut Vec<Vec<char>>) -> i32 {
    let mut paper_rolls = 0;
    let mut cur_papers = part1_paper(&grid);

    while cur_papers > 0 {
        paper_rolls += cur_papers;

        let mut remove_marks: Vec<(usize, usize)> = Vec::new();
        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() {
                if grid[i][j] == '@' && count_adjacent(&grid, i, j) < 4 {
                    remove_marks.push((i, j));
                }
            }
        }

        for (i, j) in remove_marks {
            grid[i][j] = '.';
        }

        cur_papers = part1_paper(&grid);
    }

    paper_rolls
}

fn count_adjacent(grid: &Vec<Vec<char>>, line: usize, col: usize) -> usize {
    let mut count = 0;

    for i in -1..2 {
        for j in -1..2 {
            let line = line as isize + i;
            let col = col as isize + j;
            if grid[line as usize][col as usize] == '@' {
                count += 1;
            }
        }
    }

    // don't count self
    count - 1
}
