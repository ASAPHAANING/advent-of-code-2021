use std::collections::HashMap;
use std::fmt::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

const PATH_TO_REPORT: &str = "./src/bin/day3/assets/diagnostics.txt";

fn main() {
    part1(transpose(read_file_to_vec().unwrap()));
    part2(read_file_to_vec().unwrap())
}

fn part1(diagnostics: Vec<BinaryDiagnostic>) {
    let maps = diagnostics
        .iter()
        .map(|diag| diag.into_vec().into_iter().into_group_map_by(|elem| *elem))
        .collect::<Vec<HashMap<u32, Vec<u32>>>>();

    let gamma_rate = maps
        .iter()
        .fold(BinaryDiagnostic::from_str("").unwrap(), |acc, elem| {
            let val = match elem {
                elem if elem.get(&0).unwrap().len() > elem.get(&1).unwrap().len() => 0,
                _ => 1,
            };
            BinaryDiagnostic::from_str(format!("{}{}", acc.string, val).as_str()).unwrap()
        });

    let epsilon_rate =
        gamma_rate
            .string
            .chars()
            .fold(BinaryDiagnostic::from_str("").unwrap(), |acc, char| {
                BinaryDiagnostic::from_str(
                    format!(
                        "{}{}",
                        acc.string,
                        match char {
                            char if '0' == char => "1",
                            char if '1' == char => "0",
                            _ => "",
                        }
                    )
                    .as_str(),
                )
                .unwrap()
            });

    println!("{}", gamma_rate.value * epsilon_rate.value)
}

fn part2(diagnostics: Vec<BinaryDiagnostic>) {
    let ogr_tmp = extract_rating(diagnostics.clone(), RatingType::MostCommon);
    let ogr = ogr_tmp.last().unwrap();

    let csr_tmp = extract_rating(diagnostics.clone(), RatingType::LeastCommon);
    let csr = csr_tmp.last().unwrap();

    println!("{}", ogr.value * csr.value)
}

fn extract_rating(diagnostics: Vec<BinaryDiagnostic>, rating: RatingType) -> Vec<BinaryDiagnostic> {
    (0..diagnostics[0].len()).fold(diagnostics.clone(), |iteration, idx| {
        if iteration.len() == 1 {
            return iteration;
        }

        let m = transpose(iteration.clone())
            .get(idx)
            .unwrap()
            .string
            .chars()
            .into_group_map_by(|elem| *elem);

        let zeros = m.get(&'0').map(|zeros| zeros.len()).unwrap_or(0);
        let ones = m.get(&'1').map(|ones| ones.len()).unwrap_or(0);
        let common_bit = match (zeros, rating) {
            (_, RatingType::MostCommon) if zeros > ones => '0',
            (_, RatingType::LeastCommon) if zeros > ones => '1',
            (_, RatingType::MostCommon) => '1',
            (_, RatingType::LeastCommon) => '0',
        };

        iteration
            .iter()
            .filter(|diag| {
                let curr_char = diag.string.chars().nth(idx).unwrap();
                curr_char.eq(&common_bit)
            })
            .cloned()
            .collect_vec()
    })
}

#[derive(Clone, Copy, Debug)]
enum RatingType {
    MostCommon,
    LeastCommon,
}

#[derive(Clone, Debug)]
struct BinaryDiagnostic {
    string: String,
    value: u32,
}

impl BinaryDiagnostic {
    fn from_str(str: &str) -> Result<Self, Error> {
        Ok(BinaryDiagnostic {
            string: String::from(str),
            value: u32::from_str_radix(str, 2).unwrap_or(0),
        })
    }

    fn into_vec(&self) -> Vec<u32> {
        self.string
            .chars()
            .map(|c| c.to_digit(2).unwrap())
            .collect()
    }

    fn len(&self) -> usize {
        self.string.len()
    }
}

fn read_file_to_vec() -> Result<Vec<BinaryDiagnostic>, std::io::Error> {
    Ok(BufReader::new(File::open(PATH_TO_REPORT)?)
        .lines()
        .map(|line| {
            BinaryDiagnostic::from_str(line.unwrap_or_else(|_| "".to_string()).as_str()).unwrap()
        })
        .collect())
}

fn transpose(v: Vec<BinaryDiagnostic>) -> Vec<BinaryDiagnostic> {
    (0..v[0].len())
        .map(|i| {
            v.iter()
                .map(|inner| inner.string.chars().nth(i).unwrap())
                .fold(BinaryDiagnostic::from_str("").unwrap(), |acc, char| {
                    BinaryDiagnostic::from_str(format!("{}{}", acc.string, char).as_str()).unwrap()
                })
        })
        .collect()
}
