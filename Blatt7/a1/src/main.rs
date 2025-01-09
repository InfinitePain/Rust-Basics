struct Collatz(u32);

impl Iterator for Collatz {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let current = self.0;

        // Berechne den nächsten Wert
        if current == 1 {
            self.0 = 0; // Setze auf 0 um die Sequenz zu beenden
            Some(1)
        } else if current % 2 == 0 {
            self.0 = current / 2;
            Some(current)
        } else {
            self.0 = 3 * current + 1;
            Some(current)
        }
    }
}

fn main() {
    println!("This program calculates the Collatz sequence for a given number.");
    println!("You can exit the program by entering 0.");
    loop {
        println!("Your number: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let x: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Please enter a valid number.");
                continue;
            }
        };
        if x == 0 {
            break;
        }
        println!("{:?}", Collatz(x).collect::<Vec<_>>());

        println!();
    }
    println!("See you next time!");
}
