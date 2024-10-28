mod components;
mod circuits;
mod digital_logic;
mod cli;

use cli::{
    Cli, Command
};

fn main() {
    let commands = Command::as_str();
    let mut cli = Cli::new(commands);
    cli.run();
}

