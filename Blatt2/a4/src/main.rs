use std::env;

fn main() {
    let mut exit_code = 0;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No argument provided. Default exit code: {}", exit_code);
        std::process::exit(exit_code);
    }

    match args[1].parse::<i32>() {
        Ok(num) => {
            exit_code = num;
            println!("Exit code after parsing: {}", exit_code);
            println!("Test code with 'echo $?'");
            std::process::exit(exit_code);
        }
        Err(_) => {
            println!("Error: Invalid number provided");
            std::process::exit(1);
        }
    }
}
