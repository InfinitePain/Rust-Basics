use std::sync::Arc;
use std::sync::Mutex;
use hex_interpreter::{RecordManager, Record};

#[derive(Debug)]
pub enum CommandResult {
    Exit,
    Continue(String),
}

#[derive(Debug)]
pub enum Command {
    Help,
    Exit,
    Open(String),
    Remove(String),
    ListOpen,
    List,
    Change {
        source: String,
        hash: u64,
        changes: String,
    },
    Save {
        source: String,
        target: String,
    },
    SaveBin {
        source: String,
        target: String,
    },
}

impl Command {
    pub fn as_str() -> Vec<String> {
        vec![
            "help".to_string(),
            "exit".to_string(),
            "open <file>".to_string(),
            "remove <file>".to_string(),
            "list-open".to_string(),
            "list".to_string(),
            "change <source> <hash> <changes>".to_string(),
            "save <source> <target>".to_string(),
            "save-bin <source> <target>".to_string(),
        ]
    }

    pub fn parse(input: &str) -> Option<Command> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.first().copied() {
            Some("help") => Some(Command::Help),
            Some("exit") => Some(Command::Exit),
            Some("open") if parts.len() == 2 => {
                Some(Command::Open(parts[1].to_string()))
            }
            Some("remove") if parts.len() == 2 => {
                Some(Command::Remove(parts[1].to_string()))
            }
            Some("list-open") => Some(Command::ListOpen),
            Some("list") => Some(Command::List),
            Some("change") if parts.len() >= 4 => {
                let changes = parts[3..].join(" ");
                let hash = parts[2].parse().ok()?;
                Some(Command::Change {
                    source: parts[1].to_string(),
                    hash,
                    changes,
                })
            }
            Some("save") if parts.len() == 3 => {
                Some(Command::Save {
                    source: parts[1].to_string(),
                    target: parts[2].to_string(),
                })
            }
            Some("save-bin") if parts.len() == 3 => {
                Some(Command::SaveBin {
                    source: parts[1].to_string(),
                    target: parts[2].to_string(),
                })
            }
            _ => None
        }
    }

    pub fn execute(&self, manager: Arc<Mutex<RecordManager>>) -> Result<CommandResult, String> {
        match self {
            Command::Help => Ok(CommandResult::Continue(
                "Available commands:\n\
                help - Show this message\n\
                exit - Exit the program\n\
                open <file> - Open and decode a binary record file\n\
                remove <file> - Remove an opened file from memory\n\
                list-open - List all opened files\n\
                list - List all records from all opened files\n\
                change <source> <hash> <changes> - Modify a record (format: first,last,addr,age)\n\
                save <source> <target> - Save records as text\n\
                save-bin <source> <target> - Save records as binary\n".to_string()
            )),

            Command::Exit => Ok(CommandResult::Exit),

            Command::Open(path) => {
                let mut manager = manager.lock().unwrap();
                manager.open_file(path)
                    .map_err(|e| e.to_string())
                    .map(|_| CommandResult::Continue("File opened successfully".to_string()))
            }

            Command::Remove(path) => {
                let mut manager = manager.lock().unwrap();
                if manager.remove_table(path) {
                    Ok(CommandResult::Continue("Table removed".to_string()))
                } else {
                    Err("Table not found".to_string())
                }
            }

            Command::ListOpen => {
                let manager = manager.lock().unwrap();
                let files = manager.list_open_files();
                if files.is_empty() {
                    Ok(CommandResult::Continue("No files open".to_string()))
                } else {
                    Ok(CommandResult::Continue(files.join("\n")))
                }
            }

            Command::List => {
                let manager = manager.lock().unwrap();
                let records = manager.get_all_records();
                if records.is_empty() {
                    Ok(CommandResult::Continue("No records found".to_string()))
                } else {
                    let mut output = String::new();
                    for (path, records) in records {
                        output.push_str(&format!("File: {}\n", path));
                        for record in records {
                            output.push_str(&format!(
                                "Hash: {}, Name: {} {}, Address: {}, Age: {}\n",
                                record.hash,
                                record.first_name,
                                record.last_name,
                                record.address,
                                record.age
                            ));
                        }
                        output.push('\n');
                    }
                    Ok(CommandResult::Continue(output))
                }
            }

            Command::Change { source, hash, changes } => {
                let mut manager = manager.lock().unwrap();
                let parts: Vec<&str> = changes.split(',').collect();
                if parts.len() != 4 {
                    return Err("Invalid change format. Use: first,last,addr,age".to_string());
                }

                let first_name = if parts[0].is_empty() { None } else { Some(parts[0].to_string()) };
                let last_name = if parts[1].is_empty() { None } else { Some(parts[1].to_string()) };
                let address = if parts[2].is_empty() { None } else { Some(parts[2].to_string()) };
                let age = if parts[3].is_empty() {
                    None
                } else {
                    Some(parts[3].parse::<i32>().map_err(|e| e.to_string())?)
                };

                if manager.modify_record(source, *hash, first_name, last_name, address, age) {
                    Ok(CommandResult::Continue("Record updated successfully".to_string()))
                } else {
                    Err("Record not found".to_string())
                }
            }

            Command::Save { source, target } => {
                let manager = manager.lock().unwrap();
                manager.save_table(source, target, false)
                    .map_err(|e| e.to_string())
                    .map(|_| CommandResult::Continue("File saved successfully".to_string()))
            }

            Command::SaveBin { source, target } => {
                let manager = manager.lock().unwrap();
                manager.save_table(source, target, true)
                    .map_err(|e| e.to_string())
                    .map(|_| CommandResult::Continue("File saved successfully".to_string()))
            }
        }
    }
}
