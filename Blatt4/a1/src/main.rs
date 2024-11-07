use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead};

fn choose_line() -> io::Result<String> {
    let file = File::open("random_choice_lines.txt")?;
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect();

    if lines.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "File is empty",
        ));
    }

    Ok(lines.choose(&mut rand::thread_rng())
        .expect("Lines vector was empty")
        .to_string())
}


fn main() -> io::Result<()> {
    println!("{}", choose_line()?);
    Ok(())
}
