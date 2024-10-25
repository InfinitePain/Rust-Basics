use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};

fn choose_line() -> io::Result<String> {
    let file = File::open("random_choice_lines.txt")?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    Ok(lines.choose(&mut rand::thread_rng()).unwrap().to_string())
}

fn main() -> io::Result<()> {
    println!("{}", choose_line()?);
    Ok(())
}
