use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const PATH_TO_REPORT: &str = "./src/bin/day1/assets/sonarscan.txt";

fn main() {
    let scan: Vec<i32> = read_file_to_vec().unwrap();

    part1(scan.clone());
    part2(scan);
}

fn part1(scan: Vec<i32>) {
    println!(
        "Part 1: {}",
        scan.iter()
            .zip(scan.iter().skip(1))
            .fold(0, |acc, (prev, curr)| {
                match *curr - *prev {
                    res if res > 0 => acc + 1,
                    _ => acc,
                }
            })
    );
}

fn part2(scan: Vec<i32>) {
    let iter_first: Vec<i32> = scan.windows(3).map(|x| x.iter().sum()).collect();
    let iter_next: Vec<i32> = scan.windows(3).skip(1).map(|x| x.iter().sum()).collect();

    println!(
        "Part 2: {}",
        iter_first
            .iter()
            .zip(iter_next.iter())
            .filter(|(x1, x2)| x2.gt(x1))
            .count()
    )
}

fn read_file_to_vec() -> Result<Vec<i32>, std::io::Error> {
    Ok(BufReader::new(File::open(PATH_TO_REPORT)?)
        .lines()
        .map(|l| l.unwrap_or("".to_string()))
        .map(|s| i32::from_str(s.as_str()).unwrap())
        .collect())
}
