use clap::{Arg, Command};
use std::{process, thread, time::Duration};

fn main() {
    let matches = Command::new("Exit Code Program")
        .version("1.0")
        .about("Handles exit code and delay")
        .arg(
            Arg::new("exit")
                .short('e')
                .long("exit")
                .value_name("EXIT_CODE")
                .help("Exit code to return")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("delay")
                .short('d')
                .long("delay")
                .value_name("DELAY")
                .help("Delay in seconds before exiting")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    // Parse the exit code argument
    let exit_code = matches
        .value_of("exit")
        .unwrap_or("0") // Default = exit code is 0 if not provided
        .parse::<i32>()
        .unwrap_or_else(|_| {
            eprintln!("Error: Invalid exit code provided");
            process::exit(1);
        });

    let delay = matches
        .value_of("delay")
        .unwrap_or("0") // Default = 0 seconds if not provided
        .parse::<u64>()
        .unwrap_or_else(|_| {
            eprintln!("Error: Invalid delay value provided");
            process::exit(1);
        });

    if delay > 0 {
        println!("Sleeping for {} seconds...", delay);
        thread::sleep(Duration::from_secs(delay));
    }

    println!("Exiting with code: {}", exit_code);
    process::exit(exit_code);
}
