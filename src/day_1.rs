#![allow(dead_code)]

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    ops::Sub,
};
fn get_input_from(input: impl Read) -> (Vec<i32>, Vec<i32>) {
    let buf_reader = BufReader::new(input);
    let mut first = Vec::new();
    let mut second = Vec::new();
    for line in buf_reader.lines() {
        let splitted = line
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|it| it.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        first.push(splitted[0]);
        second.push(splitted[1]);
    }
    (first, second)
}

fn how_far_apart(mut first: Vec<i32>, mut second: Vec<i32>) -> u32 {
    first.sort();
    second.sort();
    first
        .iter()
        .zip(second.iter())
        .map(|it| it.0.sub(it.1).abs() as u32)
        .sum()
}

fn find_similarity_score(first: Vec<i32>, second: Vec<i32>) -> u32 {
    let second_map = second
        .into_iter()
        .fold(HashMap::new(), |mut accum, current| {
            //
            accum.entry(current).and_modify(|e| *e += 1).or_insert(1u32);
            accum
        });
    first
        .iter()
        .map(|it| *it as u32 * second_map.get(it).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use crate::day_1::{find_similarity_score, how_far_apart};

    use super::get_input_from;

    #[test]
    fn test_with_sample_input() {
        let lists = get_input_from(
            r#"3   4
            4   3
            2   5
            1   3
            3   9
            3   3"#
                .as_bytes(),
        );
        assert_eq!(11, how_far_apart(lists.0, lists.1));
    }

    #[test]
    fn test_with_sample_input_similarity() {
        let lists = get_input_from(
            r#"3   4
            4   3
            2   5
            1   3
            3   9
            3   3"#
                .as_bytes(),
        );
        assert_eq!(31, find_similarity_score(lists.0, lists.1));
    }

    #[test]
    fn find_answer_for_first() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/input/day_1.txt");
        let file = File::open(path).unwrap();
        let lists = get_input_from(file);

        println!("Day 1 - puzzle 1 - {}", how_far_apart(lists.0, lists.1));
    }

    #[test]
    fn find_answer_for_second() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src/input/day_1.txt");
        let file = File::open(path).unwrap();
        let lists = get_input_from(file);

        println!(
            "Day 1 - puzzle 2 - {}",
            find_similarity_score(lists.0, lists.1)
        );
    }
}
