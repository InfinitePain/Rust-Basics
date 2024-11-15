mod p1;

use clap::{Arg, Command};
use std::{
    io::{self, BufRead, Write},
    process,
    thread,
    time::Duration,
};

fn main() {
    let matches = Command::new("p2")
        .version("1.0")
        .about("Test program with exit code, delay, and echo functionality")
        .arg(
            Arg::new("exit")
                .short('r')
                .long("return")
                .value_name("EXIT_CODE")
                .help("Exit code to return")
                .takes_value(true),
        )
        .arg(
            Arg::new("delay")
                .short('d')
                .long("delay")
                .value_name("DELAY")
                .help("Delay in seconds before exiting (only valid with exit code)")
                .takes_value(true)
                .requires("exit"),
        )
        .arg(
            Arg::new("echo")
                .short('e')
                .help("Echo stdin to stdout until a '.' character is entered"),
        )
        .get_matches();

    if matches.is_present("echo") {
        echo_until_dot();
        return;
    }

    if let Some(delay) = matches.value_of("delay") {
        if let Ok(delay_secs) = delay.parse::<u64>() {
            println!("Sleeping for {} seconds...", delay_secs);
            thread::sleep(Duration::from_secs(delay_secs));
        } else {
            eprintln!("Error: Invalid delay value provided");
            process::exit(1);
        }
    }

    if let Some(exit_code) = matches.value_of("exit") {
        if let Ok(code) = exit_code.parse::<i32>() {
            process::exit(code);
        } else {
            eprintln!("Error: Invalid exit code provided");
            process::exit(1);
        }
    }

    // If no arguments provided, show help
    Command::new("p2").print_help().unwrap();
    println!();
}

fn echo_until_dot() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(content) => content,
            Err(_) => {
                eprintln!("Error reading from stdin");
                break;
            }
        };
        if line.contains('.') {
            break;
        }
        if let Err(e) = writeln!(stdout, "{}", line) {
            eprintln!("Error writing to stdout: {}", e);
            break;
        }
        if let Err(e) = stdout.flush() {
            eprintln!("Error flushing stdout: {}", e);
            break;
        }
    }
}
