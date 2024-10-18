use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
struct Cmd {
    args: Vec<String>,
}

fn input(prompt: &[u8]) -> Cmd {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut cmd = String::new();
    loop {
        let e = handle.write_all(prompt); // -> Result<(), std::io::Error>
        match e {
            Ok(_) => (),
            Err(err) => panic!("error {:?}", err.kind()),
        }
        let e = handle.flush(); // -> Result<(), std::io::Error>
        match e {
            Ok(_) => (),
            Err(err) => panic!("error {:?}", err.kind()),
        }
        cmd.clear();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
        let v: Vec<&str> = cmd.trim().split(' ').collect();
        match v[0] {
            "" => continue, // just newline
            _ => {
                let mut v2: Vec<String> = vec![];
                for e in v {
                    v2.push(e.to_string());
                }
                return Cmd { args: v2 };
            }
        }
    }
}

fn vstr_to_vu32(string: Vec<String>) -> Option<Vec<u32>> {
    let mut ret: Vec<u32> = vec![];
    for s in string {
        match s.parse::<u32>() {
            Ok(num) => ret.push(num),
            Err(_) => return None,
        }
    }
    Some(ret)
}


fn vu32_to_vstr(numbers: Vec<u32>) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    for n in numbers {
        ret.push(n.to_string());
    }
    ret
}


struct PhoneBook {
    phone_book: HashMap<String, Vec<u32>>,
}

impl PhoneBook {
    fn new() -> PhoneBook {
        PhoneBook {
            phone_book: HashMap::new(),
        }
    }

    fn entry(&mut self, name: String, number: Vec<u32>) -> i8 {
        if number.is_empty() {
            return -1;
        }
        return match self.phone_book.get(&name) {
            Some(_) => {
                -2
            }
            None => {
                self.phone_book.insert(name, number);
                0
            }
        }
    }

    fn get(&self, name: &String) -> Option<&Vec<u32>> {
        self.phone_book.get(name)
    }

    fn remove(&mut self, name: &String) -> Option<Vec<u32>> {
        self.phone_book.remove(name)
    }
}

struct App {
    phone_book: PhoneBook,
}

impl App {
    fn new() -> App {
        App {
            phone_book: PhoneBook::new(),
        }
    }

    fn parse_args(&mut self, args: Vec<String>) {
        match args[0].as_str() {
            "!" => {
                if args.len() >= 3 {
                    let name = args[1].clone();
                    let numbers: Vec<u32 > = match vstr_to_vu32(args[2..].to_vec()) {
                        Some(numbers) => numbers,
                        None => {
                            println!("Fehler: nummer muss eine Zahl sein.");
                            return;
                        }
                    };
                    let ret: i8 = self.phone_book.entry(name.clone(), numbers);
                    if ret == 0 {
                        println!("Nummer für {} hinzugefügt.", name);
                    } else if ret == -1 {
                        println!("Fehler: Syntax ist '! name nummer'");
                    } else if ret == -2 {
                        println!("Fehler: Bereits ein Eintrag für {} vorhanden.", name);
                    } else {
                        println!("Fehler: Unbekannter Fehler.");
                    }
                } else {
                    println!("Fehler: Syntax ist '! name nummer'");
                }
            }
            "?" => {
                if args.len() == 2 {
                    let name = &args[1];
                    match self.phone_book.get(name) {
                        Some(numbers) => {
                            println!("{}: {}", name, vu32_to_vstr(numbers.clone()).join(", "));
                        }
                        None => {
                            println!("Kein Eintrag für {} gefunden.", name);
                        }
                    }
                } else {
                    println!("Fehler: Syntax ist '? name'");
                }
            }
            "-" => {
                if args.len() == 2 {
                    let name = &args[1];
                    match self.phone_book.remove(name) {
                        Some(_) => {
                            println!("Eintrag für {} gelöscht.", name);
                        }
                        None => {
                            println!("Kein Eintrag für {} gefunden.", name);
                        }
                    }
                } else {
                    println!("Fehler: Syntax ist '- name'");
                }
            }
            "." => {
                println!("Programm beendet.");
                std::process::exit(0);
            }
            _ => {
                println!("Unbekanntes Kommando.");
            }
        }
    }

    fn run(&mut self) {
        loop {
            let cmd = input(b"telbuch> ");
            if cmd.args.is_empty() {
                continue;
            }
            self.parse_args(cmd.args);
        }
    }
}

fn main() {
    let mut app = App::new();
    app.run();
}
