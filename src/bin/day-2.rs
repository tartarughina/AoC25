use std::{
    io::{BufRead, Result},
    path::PathBuf,
};

use aoc25::read_input;

fn digits(num: u64) -> u64 {
    let mut dig = 1;
    let mut app = num / 10;

    while app > 0 {
        dig += 1;
        app /= 10;
    }

    dig
}

/// a sequence is expressed as a series advancing by 2 after the pair 1 --> 2
/// 1 --> 2 --> 4 --> 6
fn has_digit_sequence(num: u64) -> (bool, u64) {
    let digit = digits(num);

    // skip the check if the number is a single digit or if the number is not even
    if digit == 0 || !digit.is_multiple_of(2) {
        return (false, digit);
    }

    (true, digit)
}

///  If the two halves are equal return num otherwise 0
fn compare_halves(num: u64, len: u64) -> u64 {
    let first_half = num % (10_u64.pow(len as u32));
    let second_half = num / (10_u64.pow(len as u32));

    if first_half == second_half {
        return num;
    }

    0
}

fn repeated_sequence(num: u64, digits: u64) -> u64 {
    let string = num.to_string();
    let mut sequence = string.chars();
    let stop = digits / 2;
    let mut test = String::from("");

    let mut length = 1;
    while length <= stop {
        test.push(sequence.next().unwrap());

        if digits.is_multiple_of(length) && test.repeat((digits / length) as usize) == string {
            return num;
        }

        length += 1;
    }

    0
}

///  https://adventofcode.com/2025/day/2
fn main() -> Result<()> {
    let mut input = read_input(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day-2")
            .join("input.txt"),
    )?;

    let mut line = String::from("");
    let _ = input.read_line(&mut line);

    let parsed: Vec<(u64, u64)> = line
        .trim()
        .split(",")
        .map(|x| x.split("-").collect())
        .map(|x: Vec<_>| (x[0].parse().unwrap(), x[1].parse().unwrap()))
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;

    for (start, end) in parsed {
        for test in start..=end {
            let (go, digits) = has_digit_sequence(test);

            if go {
                // println!("Checking {} long halves for {test}", digits / 2);
                part1 += compare_halves(test, digits / 2);
            }

            part2 += repeated_sequence(test, digits);
        }
    }

    println!("Part1 num {part1}");
    println!("Part2 num {part2}");

    Ok(())
}
