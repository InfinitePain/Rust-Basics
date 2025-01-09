use std::fs::File;
use std::io::Read;
use std::io::{Error, ErrorKind};
use std::{env, io};

fn task1() {
    if let Ok(mut f) = File::open("text.txt") {
        let mut s = String::new();
        let r = f
            .read_to_string(&mut s)
            .map(|_| &s as &str)
            .unwrap_or("root");
        println!("Contents of text.txt: {}", r);
    } else {
        println!("Error opening file");
    }
}

fn task2() -> Result<(), &'static str> {
    let s = vec!["apple", "mango", "banana"];
    let fourth = s.get(4).ok_or("I got only 3 fruits")?;
    println!("Fourth element: {}", fourth);
    Ok(())
}

fn task3() {
    let greeting_file = File::open("hallo.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hallo.txt")
                .unwrap_or_else(|e| panic!("Problem beim Erstellen der Datei: {:?}", e))
        } else {
            panic!("Problem beim Öffnen der Datei: {:?}", error)
        }
    });
    println!("Successfully opened hallo.txt{:?}", greeting_file);
}

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1) // Get second argument
        .ok_or("Please give at least one argument".to_owned()) // Convert None to Err
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string())) // Parse to i32
        .map(|i| i * 2) // Double the number
}

fn task5() -> Result<i32, Error> {
    if true {
        return Err(Error::new(ErrorKind::Other, "Error 1"));
    }
    if true {
        return Err(Error::new(ErrorKind::NotFound, "Error 2"));
    }
    Ok(42)
}

fn task6() -> Result<i32, Box<dyn std::error::Error>> {
    let _file = std::fs::read_to_string("nonexistent.txt")?;
    let _number = "not a number".parse::<i32>()?;
    Ok(42)
}

#[derive(Debug)]
enum MeinFehlertyp {
    IO(Error),
    Parsing(std::num::ParseIntError),
}

fn task7() -> Result<i32, MeinFehlertyp> {
    let contents = std::fs::read_to_string("nonexistent.txt").map_err(MeinFehlertyp::IO)?;
    let number = contents.parse::<i32>().map_err(MeinFehlertyp::Parsing)?;
    Ok(number)
}

impl From<std::io::Error> for MeinFehlertyp {
    fn from(err: std::io::Error) -> MeinFehlertyp {
        MeinFehlertyp::IO(err)
    }
}

impl From<std::num::ParseIntError> for MeinFehlertyp {
    fn from(err: std::num::ParseIntError) -> MeinFehlertyp {
        MeinFehlertyp::Parsing(err)
    }
}

fn main() {
    task1();
    task2().unwrap_or_else(|e| println!("Error: {}", e));
    task3();
    print!("Double of 2 is: {}", double_arg(env::args()).unwrap_or(0));
    match task5() {
        Ok(n) => println!("Task 5 success: {}", n),
        Err(e) => println!("Task 5 error: {}", e),
    }
    match task6() {
        Ok(n) => println!("Task 6 success: {}", n),
        Err(e) => println!("Task 6 error: {}", e),
    }
    match task7() {
        Ok(n) => println!("Task 7 success: {}", n),
        Err(e) => println!("Task 7 error: {:?}", e),
    }
}
