use anyhow::Result;
use parse_display::FromStr;

static INPUT_FILE_NAME: &str = "input";

#[derive(Clone, FromStr)]
#[display("{min}-{max} {letter}: {password}")]
struct InputData {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

fn main() -> Result<()> {
    let input = read_input()?;
    println!("first {}", first(input.clone()));
    println!("second {}", second(&input));
    Ok(())
}

fn first(mut input: Vec<InputData>) -> usize {
    input
        .drain(..)
        .map(
            |InputData {
                 min,
                 max,
                 letter,
                 mut password,
             }| {
                password.retain(|c| c == letter);
                password.len() >= min && password.len() <= max
            },
        )
        .filter(|b| *b)
        .count()
}

fn second(input: &[InputData]) -> usize {
    input
        .iter()
        .map(|i| {
            if let Some(a) = i.password.chars().nth(i.min - 1) {
                if let Some(b) = i.password.chars().nth(i.max - 1) {
                    let a = a == i.letter;
                    let b = b == i.letter;
                    if (!a && b) || (a && !b) {
                        return true;
                    }
                }
            }
            false
        })
        .filter(|b| *b)
        .count()
}

fn read_input() -> Result<Vec<InputData>> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(INPUT_FILE_NAME)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<InputData>().unwrap())
        .collect())
}
