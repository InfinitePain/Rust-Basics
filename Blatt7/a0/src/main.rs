use clap::Parser;

#[derive(Parser)]
#[command(name = "cli_test")]
struct Cli {
    #[arg(short = 'e')]
    exit_code: Option<i32>,
    #[arg(short = 'm')]
    message: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // Handle message printing if -m flag is present
    if let Some(msg) = cli.message {
        println!("{}", msg);
    }

    // Handle exit code if -e flag is present
    if let Some(code) = cli.exit_code {
        std::process::exit(code);
    }
}
