use crate::digital_logic::arithmetic::RippleCarryAdder;

#[derive(Debug)]
pub enum CommandResult {
    Exit,
    Continue(String),
}

#[derive(Debug)]
pub enum Command {
    Help,
    Exit,
    RippleAdd {
        bits: usize,
        num1: String,
        num2: String,
    },
}

impl Command {
    // Get string representation of command
    pub fn as_str() -> Vec<String> {
        vec![
            "help".to_string(),
            "exit".to_string(),
            "ripple <bits> <num1> <num2>".to_string(),
        ]
    }

    // Parse a string into a Command
    pub fn parse(input: &str) -> Option<Command> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.get(0).map(|s| *s) {
            Some("help") => Some(Command::Help),
            Some("exit") => Some(Command::Exit),
            Some("ripple") => {
                if parts.len() != 4 {
                    return None;
                }

                let bits = parts[1].parse().ok()?;
                let num1 = parts[2].to_string();
                let num2 = parts[3].to_string();

                Some(Command::RippleAdd { bits, num1, num2 })
            }
            _ => None
        }
    }

    // Execute the command
    pub fn execute(&self) -> Result<CommandResult, String> {
        match self {
            Command::Help => Ok(CommandResult::Continue("Available commands:\n\
                              help - Show this message\n\
                              exit - Exit the program\n\
                              ripple <bits> <num1> <num2> - Add two binary numbers using ripple carry adder where bits is the maximum number of bits sum can have\n".to_string())),

            Command::Exit => Ok(CommandResult::Exit),

            Command::RippleAdd { bits, num1, num2 } => {
                let mut adder = RippleCarryAdder::new(*bits);
                let (sum, overflow) = adder.calculate(&num1, &num2);
                Ok(CommandResult::Continue(format!("Sum: {}\n{}", sum, if overflow { "Overflow occurred!" } else { "" })))
            }
        }
    }
}