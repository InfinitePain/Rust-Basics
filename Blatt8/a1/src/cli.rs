mod command;
mod completer;

use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, EditMode, Editor};
use rustyline::hint::HistoryHinter;
use rustyline::history::DefaultHistory;
use std::sync::Arc;
use std::sync::Mutex;

pub use command::{Command, CommandResult};
pub use completer::cliHelper;
use hex_interpreter::RecordManager;

pub struct Cli {
    rl: Editor<cliHelper, DefaultHistory>,
    prompt: String,
}

impl Cli {
    pub fn new(commands: Vec<String>) -> Self {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Emacs)
            .build();

        let mut rl = Editor::with_config(config).expect("Failed to create editor");

        let helper = cliHelper {
            hinter: HistoryHinter {},
            colored_prompt: String::from("\x1b[1;32mrecord>\x1b[0m "),
            commands,
        };

        rl.set_helper(Some(helper));
        Self {
            rl,
            prompt: String::from("record> "),
        }
    }

    pub fn run(&mut self, manager: Arc<Mutex<RecordManager>>) {
        if self.rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        loop {
            match self.rl.readline(&self.prompt) {
                Ok(line) => {
                    self.rl.add_history_entry(&line).ok();
                    let command = Command::parse(&line).unwrap_or(Command::Help);
                    match command.execute(Arc::clone(&manager)) {
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
