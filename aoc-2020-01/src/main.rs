use anyhow::Result;

static INPUT_FILE_NAME: &str = "input";
static TARGET: u64 = 2020;

fn main() -> Result<()> {
    let input = read_input()?;
    match find_pair(&input) {
        Some((i, j)) => println!("found pair: {} {}, result: {}", i, j, i * j),
        None => println!("target not found"),
    }
    match find_triple(&input) {
        Some((i, j, k)) => println!("found triple: {} {} {}, result: {}", i, j, k, i * j * k),
        None => println!("target not found"),
    }
    Ok(())
}

fn find_pair(input: &[u64]) -> Option<(u64, u64)> {
    for i in input.iter() {
        for j in input.iter() {
            if i + j == TARGET {
                return Some((*i, *j));
            }
        }
    }
    None
}

fn find_triple(input: &[u64]) -> Option<(u64, u64, u64)> {
    for i in input.iter() {
        for j in input.iter() {
            for k in input.iter() {
                if i + j + k == TARGET {
                    return Some((*i, *j, *k));
                }
            }
        }
    }
    None
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
        .map(|l| l.parse::<u64>())
        .filter_map(|l| l.ok())
        .collect())
}
