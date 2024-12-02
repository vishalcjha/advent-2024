#![allow(dead_code)]

use std::io::BufRead;

fn find_safe_report_count(line_reader: impl BufRead) -> usize {
    line_reader
        .lines()
        .filter_map(|it| it.ok())
        .filter_map(|line| {
            let numbers = to_numbers(line);
            if is_safe(&numbers) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn to_numbers(line: String) -> Vec<i32> {
    line.trim()
        .split_ascii_whitespace()
        .filter_map(|it| it.parse::<i32>().ok())
        .collect::<Vec<_>>()
}

fn is_safe(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true;
    }
    if numbers[1] == numbers[0] {
        return false;
    }

    let is_increasing = numbers[1] - numbers[0] > 0;
    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        if is_increasing && diff < 0 || !is_increasing && diff > 0 {
            return false;
        }
        if diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }
    }

    true
}

fn find_safe_report_with_dampener_brute(line_reader: impl BufRead) -> usize {
    let mut safe_count = 0;
    for line in line_reader.lines().filter_map(|it| it.ok()) {
        let numbers = to_numbers(line);
        if is_safe(&numbers) {
            safe_count += 1;
            continue;
        }

        for i in 0..numbers.len() {
            let mut duplicated = numbers.clone();
            duplicated.remove(i);
            if is_safe(&duplicated) {
                safe_count += 1;
                break;
            }
        }
    }
    safe_count
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader, path::PathBuf};

    use crate::day_2::{find_safe_report_count, find_safe_report_with_dampener_brute};

    #[test]
    fn test_first_puzzle_sample() {
        assert_eq!(
            2,
            find_safe_report_count(
                r#"7 6 4 2 1
                1 2 7 8 9
                9 7 6 2 1
                1 3 2 4 5
                8 6 4 4 1
                1 3 6 7 9"#
                    .as_bytes()
            )
        );
    }

    #[test]
    fn test_first_get_answer() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/input/day_2.txt");
        let file = File::open(path).unwrap();
        let buf_reader = BufReader::new(file);
        println!("{}", find_safe_report_count(buf_reader));
    }

    #[test]
    fn test_second_puzzle_sample() {
        assert_eq!(
            4,
            find_safe_report_with_dampener_brute(
                r#"7 6 4 2 1
                1 2 7 8 9
                9 7 6 2 1
                1 3 2 4 5
                8 6 4 4 1
                1 3 6 7 9"#
                    .as_bytes()
            )
        );
    }

    #[test]
    fn test_second_get_answer() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/input/day_2.txt");
        let file = File::open(path).unwrap();
        let buf_reader = BufReader::new(file);
        println!("{}", find_safe_report_with_dampener_brute(buf_reader));
    }
}
