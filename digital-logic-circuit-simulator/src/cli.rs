mod command;
mod completer;

use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, EditMode, Editor};
use rustyline::hint::HistoryHinter;
use rustyline::history::DefaultHistory;

pub use command::{Command, CommandResult};
pub use completer::SimulatorHelper;

pub struct Cli {
    rl: Editor<SimulatorHelper, DefaultHistory>,
    prompt: String,
}

impl Cli {
    pub fn new(commands: Vec<String>) -> Self {
        // Configure the editor
        let config = Config::builder()
            .history_ignore_space(true)
            .edit_mode(EditMode::Emacs)
            .completion_type(CompletionType::List)
            .build();

        let mut rl = Editor::with_config(config).expect("Failed to create editor");

        let helper = SimulatorHelper {
            hinter: HistoryHinter {},
            colored_prompt: String::from("\x1b[1;32msimulator>\x1b[0m "),
            commands,
        };
       
        rl.set_helper(Some(helper));
        Self { rl, prompt: String::from("simulator> ") }
    }


    pub fn run(&mut self) {
        if self.rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        println!("Digital Logic Circuit Simulator");
        println!("Type 'help' for commands or 'exit' to quit");
        println!("Use Tab for completion and arrow keys for history");

        loop {
            match self.rl.readline(&self.prompt) {
                Ok(line) => {
                    self.rl.add_history_entry(&line).ok();
                    let command = Command::parse(&line).unwrap_or(Command::Help);
                    match command.execute() {
                        Ok(CommandResult::Continue(output)) => println!("{}", output),
                        Ok(CommandResult::Exit) => break,
                        Err(err) => println!("Error: {}", err),
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Received Ctrl-C, exiting...");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        self.rl.save_history("history.txt").expect("Failed to save history");
    }
}
