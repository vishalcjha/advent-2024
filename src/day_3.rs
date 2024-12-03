#![allow(dead_code)]

use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, Read},
};

use regex::Regex;
fn mul_it_over(read: impl Read) -> i64 {
    let line = BufReader::new(read).lines().next().unwrap().unwrap();
    let re = Regex::new("mul\\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\\)").unwrap();
    let mut total = 0;

    for matched in re.find_iter(&line).into_iter() {
        println!("{:?}", matched);
        let mut splitted = matched.as_str().split(",").into_iter();
        let first = &splitted.next().unwrap()[4..];
        let second = splitted.next().unwrap().to_string();
        let second = second.split(")").next().unwrap();

        let first = first.parse::<i64>().unwrap();
        let second = second.parse::<i64>().unwrap();

        total += first * second;
    }
    total
}

fn mul_it_over_with_instruction(read: impl Read) -> i64 {
    let line = BufReader::new(read).lines().next().unwrap().unwrap();
    let re = Regex::new("mul\\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\\)").unwrap();
    let pattern = r"do\(\)|don't\(\)";
    let instruction = Regex::new(pattern).unwrap();
    let mut instructions = instruction
        .find_iter(&line)
        .into_iter()
        .collect::<VecDeque<_>>();

    let mut total = 0;

    let mut accept = true;
    for matched in re.find_iter(&line).into_iter() {
        let begin_of_mul = matched.start();
        loop {
            if !instructions.is_empty() {
                if instructions[0].start() < begin_of_mul {
                    let instruction = instructions.pop_front().unwrap();
                    accept = instruction.as_str().eq("do()");
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if !accept {
            continue;
        }

        let mut splitted = matched.as_str().split(",").into_iter();
        let first = &splitted.next().unwrap()[4..];
        let second = splitted.next().unwrap().to_string();
        let second = second.split(")").next().unwrap();

        let first = first.parse::<i64>().unwrap();
        let second = second.parse::<i64>().unwrap();

        total += first * second;
    }
    total
}

#[cfg(test)]
mod test {
    use std::{fs::File, path::PathBuf};

    use crate::day_3::{mul_it_over, mul_it_over_with_instruction};

    #[test]
    fn test_first_sample_input() {
        assert_eq!(
            161,
            mul_it_over(
                r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#
                    .as_bytes()
            )
        );
    }

    #[test]
    fn test_first_puzzle() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/input/day_3.txt");
        let file = File::open(path).unwrap();

        println!("{}", mul_it_over(file));
    }

    #[test]
    fn test_second_sample_input() {
        assert_eq!(
            48,
            mul_it_over_with_instruction(
                r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#
                    .as_bytes()
            )
        );
    }

    #[test]
    fn test_second_puzzle() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/input/day_3.txt");
        let file = File::open(path).unwrap();

        println!("{}", mul_it_over_with_instruction(file));
    }
}
