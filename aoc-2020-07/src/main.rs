use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

static INPUT_FILE_NAME: &str = "input";

type Input = HashMap<String, HashMap<String, usize>>;

fn main() -> Result<()> {
    let input = read_input()?;
    println!("first: {:?}", first(&input));
    println!("second: {:?}", second(&input));
    Ok(())
}

fn second(input: &Input) -> usize {
    contained_bags(input, "shiny gold")
}

fn contained_bags(input: &Input, bag: &str) -> usize {
    input
        .get(bag)
        .unwrap()
        .iter()
        .map(|(b, n)| n * (contained_bags(input, b) + 1))
        .sum()
}

fn first(input: &Input) -> usize {
    input
        .iter()
        .filter(|(b, _)| b.as_str() != "shiny gold" && contains_shiny_gold(input, &b))
        .count()
}

fn contains_shiny_gold(input: &Input, bag: &str) -> bool {
    if bag == "shiny gold" {
        return true;
    } else {
        input
            .get(bag)
            .unwrap()
            .iter()
            .map(|(b, _)| contains_shiny_gold(input, b))
            .any(|b| b)
    }
}

fn read_input() -> Result<Input> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let contained_color = Regex::new("\\d\\s\\w+\\s\\w+").unwrap();
    let bag_color = Regex::new("^\\w+\\s\\w+").unwrap();
    let mut rules = HashMap::new();
    let file = File::open(INPUT_FILE_NAME)?;
    BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .for_each(|l| {
            let mut contained = HashMap::new();
            contained_color.find_iter(&l).for_each(|c| {
                contained.insert(
                    l.get(c.start() + 2..c.end()).unwrap().into(),
                    l.get(c.start()..c.start() + 1)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                );
            });
            let bag = bag_color.find(&l).unwrap();
            rules.insert(l.get(bag.start()..bag.end()).unwrap().into(), contained);
        });
    Ok(rules)
}
