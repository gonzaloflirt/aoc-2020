use anyhow::Result;
use std::collections::HashSet;

static INPUT_FILE_NAME: &str = "input";

type Input = Vec<Vec<HashSet<char>>>;

fn main() -> Result<()> {
    let input = read_input()?;
    println!("first: {:?}", first(&input));
    println!("second: {:?}", second(&input));
    Ok(())
}

fn first(input: &Input) -> usize {
    input
        .iter()
        .map(|g| g.iter().flatten().collect::<HashSet<&char>>().len())
        .sum()
}

fn second(input: &Input) -> usize {
    input
        .iter()
        .map(|group| {
            let mut count = 0;
            for c in group.first().expect("empty group").iter() {
                if group.iter().filter(|p| p.contains(c)).count() == group.len() {
                    count += 1;
                }
            }

            count
        })
        .sum()
}

fn read_input() -> Result<Input> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let mut answers = vec![];
    let mut group = vec![];

    let file = File::open(INPUT_FILE_NAME)?;
    for line in BufReader::new(file).lines().filter_map(|l| l.ok()) {
        if line.is_empty() {
            answers.push(group);
            group = vec![];
        } else {
            let mut person = HashSet::new();
            for c in line.chars() {
                person.insert(c);
            }
            group.push(person);
        }
    }
    Ok(answers)
}
