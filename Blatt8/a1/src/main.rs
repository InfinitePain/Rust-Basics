use std::sync::Arc;
use std::sync::Mutex;
mod cli;

use cli::{Cli, Command};

use hex_interpreter::RecordManager;

fn main() {
    let manager = Arc::new(Mutex::new(RecordManager::default()));
    let commands = Command::as_str();
    let mut cli = cli::Cli::new(commands);

    println!("Record Management System");
    println!("Type 'help' for commands or 'exit' to quit");

    cli.run(manager);
}
