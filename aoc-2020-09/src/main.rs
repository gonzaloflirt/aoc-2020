use anyhow::Result;

static INPUT_FILE_NAME: &str = "input";

fn main() -> Result<()> {
    let input = read_input()?;
    let first = first(25, &input).unwrap();
    println!("first: {:?}", first);
    println!("second: {:?}", second(first, &input).unwrap());
    Ok(())
}

fn second(target: u64, input: &[u64]) -> Option<u64> {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let slice = &input[i..j];
            if slice.iter().sum::<u64>() == target {
                return Some(slice.iter().min().unwrap() + slice.iter().max().unwrap());
            }
        }
    }
    None
}

fn first(preamble: usize, input: &[u64]) -> Option<u64> {
    for i in preamble..input.len() {
        if !contains_sum(input[i], &input[i - preamble..i]) {
            return Some(input[i]);
        }
    }
    None
}

fn contains_sum(sum: u64, data: &[u64]) -> bool {
    for (i_a, a) in data.iter().enumerate() {
        for (i_b, b) in data.iter().enumerate() {
            if i_a != i_b && a + b == sum {
                return true;
            }
        }
    }

    false
}

fn read_input() -> Result<Vec<u64>> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(INPUT_FILE_NAME)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u64>().unwrap())
        .collect())
}
