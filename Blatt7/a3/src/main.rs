use itertools::Itertools;

/// Berechnet den Modus (häufigste Zahl) aus einem sortierten Vektor ganzer Zahlen.
///
/// # Funktionsweise
/// Die Funktion verwendet mehrere Iterator-Transformationen:
/// 1. `iter()` - Erstellt einen Iterator über Referenzen zu den Zahlen
/// 2. `copied()` - Kopiert die Werte (dereferenziert die Referenzen)
/// 3. `group_by()` - Gruppiert aufeinanderfolgende gleiche Zahlen
/// 4. `into_iter()` - Wandelt die Gruppierung in einen Iterator um
/// 5. `map()` - Transformiert jede Gruppe in ein Tupel (Zahl, Häufigkeit)
/// 6. `max_by_key()` - Findet das Tupel mit der höchsten Häufigkeit
/// 7. `unwrap()` - Extrahiert das Ergebnis aus dem Option
/// 8. `.0` - Gibt die Zahl aus dem Tupel zurück (erste Komponente)
///
/// # Panics
/// Die Funktion panic!t, wenn der Eingabevektor leer ist.
fn find_mode(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .copied()
        .group_by(|x| *x)
        .into_iter()
        .map(|(k, v)| (k, v.count()))
        .max_by_key(|(_, v)| *v)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_mode() {
        // Ein einzelner klarer Modus
        let numbers = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(find_mode(&numbers), 1);
    }

    #[test]
    fn test_multiple_modes_returns_first() {
        // Bei mehreren Modi wird der erste zurückgegeben
        let numbers = vec![1, 1, 2, 2, 3, 3];
        assert_eq!(find_mode(&numbers), 3);
    }

    #[test]
    fn test_single_element() {
        // Ein einzelnes Element ist sein eigener Modus
        let numbers = vec![42];
        assert_eq!(find_mode(&numbers), 42);
    }

    #[test]
    fn test_negative_numbers() {
        // Funktioniert auch mit negativen Zahlen
        let numbers = vec![-3, -3, -2, -1, 0];
        assert_eq!(find_mode(&numbers), -3);
    }

    #[test]
    #[should_panic]
    fn test_empty_vector_panics() {
        // Ein leerer Vektor sollte panic!en
        let numbers: Vec<i32> = vec![];
        find_mode(&numbers);
    }
}
