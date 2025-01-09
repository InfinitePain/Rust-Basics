use std::str::FromStr;

fn parse_list(input: &str) -> Result<Vec<Vec<u32>>, <u32 as FromStr>::Err> {
    input
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse())
                .collect()
        })
        .collect()
}

fn parse_list_short(input: &str) -> Result<Vec<Vec<u32>>, <u32 as FromStr>::Err> {
    input.split(';')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.split(',')
            .filter(|n| !n.trim().is_empty())
            .map(|n| n.trim().parse())
            .collect()
        )
        .collect()
}

fn main() {
    // 1,2,3; 4,5,6
    let input = "1,2,3; 4,5,6";
    match parse_list(input) {
        Ok(result) => println!("parse_list: {:?}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    match parse_list_short(input) {
        Ok(result) => println!("parse_list_short: {:?}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        assert_eq!(
            parse_list("1,2,3; 4,5,6").unwrap(),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );
        assert_eq!(
            parse_list_short("1,2,3; 4,5,6").unwrap(),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );

        // Test mit Leerzeichen und leeren Teilen
        assert_eq!(
            parse_list("1, 2,3;  ; 4,5  ,6").unwrap(),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );
        assert_eq!(
            parse_list_short("1, 2,3;  ; 4,5  ,6").unwrap(),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );

        // Test mit einzelnem Element
        assert_eq!(
            parse_list("42").unwrap(),
            vec![vec![42]]
        );
        assert_eq!(
            parse_list_short("42").unwrap(),
            vec![vec![42]]
        );
    }

    #[test]
    fn test_invalid_input() {
        // Test mit ungültiger Zahl
        assert!(parse_list("1,2,abc").is_err());
        assert!(parse_list_short("1,2,abc").is_err());

        // Test mit zu großer Zahl für u32
        assert!(parse_list("1,2,4294967296").is_err());
        assert!(parse_list_short("1,2,4294967296").is_err());
    }
}