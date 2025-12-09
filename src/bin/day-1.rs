use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::PathBuf,
};

use aoc25::read_input;

/// https://adventofcode.com/2025/day/1
fn main() -> std::io::Result<()> {
    println!("Day 1 challenge");

    let lines: Lines<BufReader<File>> =
        read_input(PathBuf::from("inputs").join("day-1").join("input"))?.lines();

    let mut pos: i32 = 50;
    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    lines.filter_map(|x| x.ok()).for_each(|mut x| {
        // String has the format POS:DIALS
        // POS is 1 char long

        let dials: i32 = x.split_off(1).parse().unwrap();
        let mode = x;

        if mode == "L" {
            let mut app = pos - dials;

            let mut met_zero = if app.signum() + pos.signum() == 0 {
                1
            } else {
                0
            };

            met_zero =
                met_zero + (app / 100).abs() + if app % 100 == 0 && app != 0 { -1 } else { 0 };

            app %= 100;

            if app.is_negative() {
                pos = 100 + app;
            } else {
                pos = app;
            }

            part2 += met_zero as u32;
        } else {
            let app = pos + dials;

            pos = app % 100;
            let met_zero = app / 100 + if pos == 0 { -1 } else { 0 };

            part2 += met_zero as u32;
        }

        if pos == 0 {
            part1 += 1;
            part2 += 1;
        }
    });

    println!("Part1 psw {part1}");
    println!("Part2 psw {part2}");

    Ok(())
}
