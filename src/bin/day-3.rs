use std::{
    io::{BufRead, Result},
    path::PathBuf,
    vec,
};

use aoc25::read_input;

///  https://adventofcode.com/2025/day/3
fn main() -> Result<()> {
    let lines = read_input(PathBuf::from("inputs").join("day-3").join("input.txt"))?.lines();

    let solutions = lines
        .filter_map(|x| x.ok())
        .map(|x: String| x.char_indices().collect())
        .map(|chars: Vec<_>| {
            let mut max_l: usize = 0;
            let mut max_r: usize = chars.len() - 1;

            for i in 0..max_r {
                if chars[i].1 > chars[max_l].1 {
                    max_l = i;
                }
            }

            for i in (max_l + 1..=max_r).rev() {
                if chars[i].1 > chars[max_r].1 {
                    max_r = i;
                }
            }

            let mut to_remove = chars.len() - 12;
            let mut stack: Vec<char> = vec![];

            for j in &chars {
                while stack.last().is_some() && Some(&j.1) > stack.last() && to_remove > 0 {
                    stack.pop();
                    to_remove -= 1;
                }

                stack.push(j.1);
            }

            for _ in 0..to_remove {
                stack.pop();
            }

            let part2 = stack
                .iter()
                .fold(String::from(""), |acc, x| format!("{}{}", acc, x))
                .parse::<u64>()
                .unwrap_or(0);

            (
                format!("{}{}", chars[max_l].1, chars[max_r].1),
                format!("{part2}"),
            )
        })
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u64>().unwrap()))
        .reduce(|acc: (u32, u64), e: (u32, u64)| (acc.0 + e.0, acc.1 + e.1))
        .unwrap();

    println!("Part1: {}", solutions.0);
    println!("Part2: {}", solutions.1);

    Ok(())
}
