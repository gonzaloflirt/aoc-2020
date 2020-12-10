use anyhow::Result;

static INPUT_FILE_NAME: &str = "input";

fn main() -> Result<()> {
    let mut input = read_input()?;
    input.push(0);
    input.push(input.iter().max().unwrap() + 3);
    input.sort_unstable();
    println!("first: {:?}", first(&input));
    println!("second: {:?}", second(&input));
    Ok(())
}

fn second(input: &[u64]) -> u64 {
    input
        .windows(2)
        .collect::<Vec<_>>()
        .split(|n| n[1] - n[0] == 3)
        .map(|n| match n.len() {
            4 => 7,
            3 => 4,
            2 => 2,
            _ => 1,
        })
        .product()
}

fn first(input: &[u64]) -> u64 {
    let diffs = input.windows(2).map(|s| s[1] - s[0]).collect::<Vec<_>>();
    let ones = diffs.iter().filter(|i| 1 == **i).count() as u64;
    let threes = diffs.iter().filter(|i| 3 == **i).count() as u64;
    ones * threes
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
