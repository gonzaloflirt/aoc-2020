use anyhow::{anyhow, Result};

static INPUT_FILE_NAME: &str = "input";

#[derive(Clone, Debug)]
enum Cmd {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn from_str(data: &str) -> Result<Cmd> {
    let mut parts = data.split(' ');
    let cmd = parts.next().unwrap().into();
    let arg = parts.next().unwrap().parse::<i32>().unwrap();
    match cmd {
        Some("nop") => Ok(Cmd::Nop(arg)),
        Some("acc") => Ok(Cmd::Acc(arg)),
        Some("jmp") => Ok(Cmd::Jmp(arg)),
        _ => {
            println!("{:?}", data);
            Err(anyhow!("invalid input"))
        }
    }
}

fn main() -> Result<()> {
    let input = read_input()?;
    println!("first: {:?}", run(&input).1);
    println!("second: {:?}", fix(&input)?);
    Ok(())
}

fn fix(input: &[Cmd]) -> Result<i32> {
    for i in 0..input.len() {
        if let Cmd::Jmp(a) = input[i] {
            let mut input = input.to_owned();
            input[i] = Cmd::Nop(a);
            let (ter, acc) = run(&input);
            if !ter {
                return Ok(acc);
            }
        } else if let Cmd::Nop(a) = input[i] {
            let mut input = input.to_owned();
            input[i] = Cmd::Jmp(a);
            let (ter, acc) = run(&input);
            if !ter {
                return Ok(acc);
            }
        }
    }
    Err(anyhow!("no fix :("))
}

fn run(input: &[Cmd]) -> (bool, i32) {
    let mut prog = input.iter().map(|cmd| (cmd, false)).collect::<Vec<_>>();
    let mut acc = 0;
    let mut pos = 0;
    loop {
        if pos == input.len() {
            return (false, acc);
        } else if prog[pos].1 {
            return (true, acc);
        } else {
            prog[pos].1 = true;
            match prog[pos].0 {
                Cmd::Nop(_) => pos += 1,
                Cmd::Acc(a) => {
                    pos += 1;
                    acc += a;
                }
                Cmd::Jmp(a) => pos = (pos as i32 + a) as usize,
            }
        }
    }
}

fn read_input() -> Result<Vec<Cmd>> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(INPUT_FILE_NAME)?;
    Ok(BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.is_empty())
        .map(|l| from_str(&l).unwrap())
        .collect())
}
