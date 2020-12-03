use anyhow::Result;

static INPUT_FILE_NAME: &str = "input";

fn main() -> Result<()> {
    let input = read_input()?;

    println!("first {}", count(&input, 1, 3));

    println!(
        "second {}",
        [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
            .iter()
            .map(|(dx, dy)| { count(&input, *dx, *dy) })
            .product::<usize>()
    );

    Ok(())
}

fn count(i: &Vec<Vec<bool>>, dx: usize, dy: usize) -> usize {
    let mut y = 0;
    i.iter()
        .step_by(dx)
        .map(|l| {
            let b = l[y];
            y = (y + dy) % l.len();
            b
        })
        .filter(|b| *b)
        .count()
}

fn read_input() -> Result<Vec<Vec<bool>>> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(INPUT_FILE_NAME)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect())
}
