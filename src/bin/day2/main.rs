mod common;
mod p1;
mod p2;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::common::Commandable;
use common::Command;

const PATH_TO_REPORT: &str = "./src/bin/day2/assets/instructions.txt";

fn main() {
    let cmds = read_file_to_vec().unwrap();
    part1(cmds.clone());
    part2(cmds);
}

fn part1(cmds: Vec<Command>) {
    println!(
        "Part 1: {}",
        cmds.into_iter()
            .fold(p1::Submarine::default(), |state, cmd| {
                state.navigate(cmd)
            })
            .get_position()
    )
}

fn part2(cmds: Vec<Command>) {
    println!(
        "Part 2: {}",
        cmds.into_iter()
            .fold(p2::Submarine::default(), |state, cmd| {
                state.navigate(cmd)
            })
            .get_position()
    )
}

fn read_file_to_vec() -> Result<Vec<Command>, std::io::Error> {
    Ok(BufReader::new(File::open(PATH_TO_REPORT)?)
        .lines()
        .map(|l| l.unwrap_or("".to_string()))
        .flat_map(|l| {
            l.split_whitespace()
                .take(2)
                .collect::<Vec<&str>>()
                .windows(2)
                .map(|v| match v[0] {
                    "forward" => Command::FORWARD(i32::from_str(v[1]).unwrap_or(0)),
                    "down" => Command::DOWN(i32::from_str(v[1]).unwrap_or(0)),
                    "up" => Command::UP(i32::from_str(v[1]).unwrap_or(0)),
                    _ => Command::NOOP,
                })
                .collect::<Vec<Command>>()
        })
        .collect())
}
