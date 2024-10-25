use clap::{Arg, Command};
use std::{
    io::{self, BufRead, Write},
    process::{self, Command as ProcessCommand, Stdio},
    thread,
    time::Duration,
};

fn main() {
    // Define the CLI using `Command`
    let mut cmd = Command::new("Exit Code Program")
        .version("1.0")
        .about("Handles exit code, delay, and echo")
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
                .help("Delay in seconds before exiting")
                .takes_value(true),
        )
        .arg(
            Arg::new("echo")
                .short('e')
                .help("Echo stdin to stdout until a '.' character is entered"),
        )
        .arg(
            Arg::new("program")
                .short('p')
                .long("program")
                .value_name("PROGRAM")
                .help("Path to program to start and communicate with")
                .takes_value(true),
        );

    // Parse the arguments
    let matches = cmd.clone().get_matches();

    // Determine if no arguments were provided
    let should_show_help = !matches.is_present("exit")
        && !matches.is_present("delay")
        && !matches.is_present("echo")
        && !matches.is_present("program");

    if should_show_help {
        cmd.print_help().expect("Failed to print help message");
        println!();
        process::exit(0);
    }

    // Handle delay
    if let Some(delay) = matches.value_of("delay") {
        if let Ok(delay_secs) = delay.parse::<u64>() {
            println!("Sleeping for {} seconds...", delay_secs);
            thread::sleep(Duration::from_secs(delay_secs));
        } else {
            eprintln!("Error: Invalid delay value provided");
            process::exit(1);
        }
    }

    // Handle echo
    if matches.is_present("echo") {
        echo_until_dot();
    }

    // Handle exit code
    if let Some(exit_code) = matches.value_of("exit") {
        if let Ok(code) = exit_code.parse::<i32>() {
            process::exit(code);
        } else {
            eprintln!("Error: Invalid exit code provided");
            process::exit(1);
        }
    }

    // Handle program execution
    if let Some(program) = matches.value_of("program") {
        start_and_communicate_with_program(program);
    }
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

fn start_and_communicate_with_program(path: &str) {
    let mut child = ProcessCommand::new(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
    let child_stdout = child.stdout.take().expect("Failed to open stdout");

    // Send messages to the child program
    let messages = vec!["Hello", "Rust", "World", "."];
    for msg in messages {
        if let Err(e) = writeln!(child_stdin, "{}", msg) {
            eprintln!("Failed to write to child stdin: {}", e);
            break;
        }
    }

    // Read responses from the child program
    let reader = io::BufReader::new(child_stdout);
    for line in reader.lines() {
        match line {
            Ok(content) => println!("Received from program: {}", content),
            Err(e) => {
                eprintln!("Failed to read line from child stdout: {}", e);
                break;
            }
        }
    }

    let _ = child.wait();
}
