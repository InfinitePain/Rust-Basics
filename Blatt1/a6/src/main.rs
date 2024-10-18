fn print_calc_collatz(number: u32) {
    let mut number = number;
    if number == 0 {
        println!("Error: Number is 0");
        return;
    }
    print!("{}", number);
    while number != 1 {
        print!(", ");
        if number % 2 == 0 {
            number = number / 2;
        } else {
            number = 3 * number + 1;
        }
        print!("{}", number);
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
        print_calc_collatz(x);
        println!();
    }
    println!("See you next time!");
}
