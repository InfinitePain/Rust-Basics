use std::collections::HashMap;
pub mod input;


pub fn mean(numbers: &Vec<i32>) -> f32 {
    let mut sum : i32 = 0;
    for number in numbers {
        sum += number;
    }
    sum as f32 / numbers.len() as f32
}

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();
    let len = numbers.len();
    if len % 2 == 0 {
        (numbers[len / 2 - 1] + numbers[len / 2]) as f32 / 2.0
    } else {
        numbers[len / 2] as f32
    }
}

pub fn mode(numbers: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();
    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Unable to find mode")
}

fn main() {
    let mut numbers = input::input_ints(Some("Bitte geben Sie mehrere ganze Zahlen ein, getrennt durch Leerzeichen: "));

    if numbers.is_empty() {
        eprintln!("Es wurden keine Zahlen eingegeben.");
        return;
    }

    // Berechne den Mittelwert
    let mean = mean(&numbers);

    // Berechne den Median
    let median = median(&mut numbers);

    // Berechne den Modus (häufigster Wert)
    let mode = mode(&numbers);

    // Ergebnisse ausgeben
    println!("Mittelwert: {:.2}", mean);
    println!("Median: {:.2}", median);
    println!("Modus: {}", mode);
}

