use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const PATH_TO_REPORT: &str = "./src/bin/day4/assets/input.txt";
const DIMENSION: usize = 5;

fn main() {
    let (prompts, boards) = read_input().expect("input to be properly deserialized");

    part1(prompts, boards)
}

fn part1(prompts: Vec<u32>, boards: Vec<Board>) {
    let (boards_with_winning_board, prompt) = prompts
        .iter()
        .fold_while(
            (
                boards,
                *prompts.first().expect("expected at least one prompt"),
            ),
            |(b, _), prompt| {
                let prompted_boards = b
                    .iter()
                    .map(|board| board.set_fields(*prompt))
                    .collect_vec();

                match prompted_boards.iter().find(|board| board.has_won()) {
                    None => Continue((prompted_boards, *prompt)),
                    Some(_) => Done((prompted_boards, *prompt)),
                }
            },
        )
        .into_inner();

    let winning_board = boards_with_winning_board
        .iter()
        .find(|board| board.has_won());

    println!(
        "{}",
        winning_board
            .expect("at least one winning board")
            .get_result(prompt)
    )
}

#[derive(Clone, Debug)]
struct Board {
    fields: Vec<Vec<Field>>,
}

#[derive(Clone, Debug)]
struct Field {
    number: u32,
    marked: bool,
}

impl Board {
    fn set_fields(&self, prompt: u32) -> Self {
        Board {
            fields: self
                .fields
                .iter()
                .map(|fields| {
                    fields
                        .iter()
                        .cloned()
                        .map(|field| match field.number {
                            num if num == prompt => Field {
                                marked: true,
                                ..field
                            },
                            _ => Field { ..field },
                        })
                        .collect_vec()
                })
                .collect_vec(),
        }
    }

    fn has_won(&self) -> bool {
        self.fields.iter().any(Self::is_row_marked)
            || transpose(self.fields.clone())
                .iter()
                .any(Self::is_row_marked)
    }

    fn is_row_marked(fields: &Vec<Field>) -> bool {
        fields.iter().all(|f| f.marked)
    }

    fn get_result(&self, winning_prompt: u32) -> u32 {
        let sum_of_unmarked = &self.fields.iter().fold(0, |acc, fields| {
            acc + fields.iter().fold(0, |acc, field| match field.marked {
                false => field.number + acc,
                true => acc,
            })
        });

        winning_prompt * sum_of_unmarked
    }
}

fn read_input() -> Result<(Vec<u32>, Vec<Board>), std::io::Error> {
    let lines = BufReader::new(File::open(PATH_TO_REPORT)?)
        .lines()
        .map(|x| x.expect("expected a string"))
        .collect_vec();

    let prompts = read_prompts(lines.clone());
    let boards = read_boards().unwrap();

    Ok((prompts, boards))
}

fn read_prompts(lines: Vec<String>) -> Vec<u32> {
    lines
        .iter()
        .take(1)
        .flat_map(|str| str.split(',').collect_vec())
        .map(|str| u32::from_str(str).expect("expected a number"))
        .collect_vec()
}

fn read_boards() -> Result<Vec<Board>, std::io::Error> {
    Ok(BufReader::new(File::open(PATH_TO_REPORT)?)
        .lines()
        .skip(2)
        .map(|x| x.expect("expected a string"))
        .map(|x1| {
            x1.split_whitespace()
                .map(|x2| u32::from_str(x2).expect("expected unsigned numbers"))
                .map(|x3| Field {
                    marked: false,
                    number: x3,
                })
                .collect_vec()
        })
        .fold(vec![Board { fields: vec![] }], |acc: Vec<Board>, fields| {
            if acc.last().unwrap().fields.len() == DIMENSION && fields.is_empty() {
                return acc
                    .iter()
                    .chain(vec![&Board { fields: vec![] }])
                    .cloned()
                    .collect_vec();
            }

            acc.iter()
                .map(|board| Board {
                    fields: match board.fields.len() {
                        f if f < DIMENSION => board
                            .fields
                            .iter()
                            .cloned()
                            .chain(vec![fields.clone()])
                            .collect_vec(),
                        _ => board.fields.clone(),
                    },
                })
                .collect_vec()
        }))
}

// Assumes NxN
fn transpose(v: Vec<Vec<Field>>) -> Vec<Vec<Field>> {
    (0..v[0].len())
        .map(|idx| {
            v.iter()
                .map(|fields| fields.get(idx).unwrap())
                .cloned()
                .collect_vec()
        })
        .collect()
}
