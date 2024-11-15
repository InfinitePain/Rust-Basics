use clap::{Arg, Command};
use std::{
    io::{self, BufRead, Write},
    process::{Command as ProcessCommand, Stdio},
    thread,
    time::Duration,
};

fn main() {
    let matches = Command::new("p1")
        .version("1.0")
        .about("Test program to run p2 with different options")
        .arg(
            Arg::new("program")
                .short('p')
                .long("program")
                .value_name("P2_PATH")
                .help("Path to p2 executable")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let p2_path = matches.value_of("program").unwrap();

    // Test 1: Exit code
    println!("\nTest 1: Testing exit code...");
    let status = ProcessCommand::new(p2_path)
        .arg("-r")
        .arg("42")
        .status()
        .expect("Failed to execute p2");
    println!("Exit status: {}", status);

    // Test 2: Delay with exit code
    println!("\nTest 2: Testing delay with exit code...");
    let status = ProcessCommand::new(p2_path)
        .arg("-r")
        .arg("0")
        .arg("-d")
        .arg("2")
        .status()
        .expect("Failed to execute p2");
    println!("Exit status after delay: {}", status);

    // Test 3: Echo functionality
    println!("\nTest 3: Testing echo functionality...");
    let mut child = ProcessCommand::new(p2_path)
        .arg("-e")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start p2");

    let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
    let child_stdout = child.stdout.take().expect("Failed to open stdout");

    // Test messages
    let test_messages = vec!["Hello", "Testing", "Echo", "."];

    // Spawn a thread to read responses
    let reader_thread = thread::spawn(move || {
        let reader = io::BufReader::new(child_stdout);
        for line in reader.lines() {
            match line {
                Ok(content) => println!("Received: {}", content),
                Err(e) => {
                    eprintln!("Error reading from p2: {}", e);
                    break;
                }
            }
        }
    });

    // Send test messages
    for msg in test_messages {
        thread::sleep(Duration::from_millis(100));
        if let Err(e) = writeln!(child_stdin, "{}", msg) {
            eprintln!("Failed to write to p2: {}", e);
            break;
        }
    }

    let _ = reader_thread.join();
    let _ = child.wait();
}