use anyhow::Result;

static INPUT_FILE_NAME: &str = "input";

fn main() -> Result<()> {
    let input = read_input()?;
    let mut seat_ids = collect_seat_ids(&input);
    seat_ids.sort();
    println!("first: {:?}", seat_ids.last().unwrap());
    println!("second: {:?}", my_seat(&seat_ids).unwrap());
    Ok(())
}

fn my_seat(seat_ids: &Vec<u32>) -> Option<u32> {
    let mut it = seat_ids.iter().peekable();
    while let Some(curr) = it.next() {
        if Some(&&(curr + 2)) == it.peek() {
            return Some(curr + 1);
        }
    }
    None
}

fn collect_seat_ids(input: &Vec<String>) -> Vec<u32> {
    input
        .iter()
        .map(|l| parse_line(&l))
        .map(|(r, c)| seat_id(r, c))
        .collect()
}

fn seat_id(column: u32, row: u32) -> u32 {
    column * 8 + row
}

fn parse_line(line: &str) -> (u32, u32) {
    let mut row = 0;
    let mut column = 0;
    let mut l = line.chars();
    for _ in 0..7 {
        row = row << 1;
        if Some('B') == l.next() {
            row = row + 1;
        }
    }
    for _ in 0..3 {
        column = column << 1;
        if Some('R') == l.next() {
            column = column + 1;
        }
    }
    (row, column)
}

fn read_input() -> Result<Vec<String>> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(INPUT_FILE_NAME)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .collect::<Vec<String>>())
}
