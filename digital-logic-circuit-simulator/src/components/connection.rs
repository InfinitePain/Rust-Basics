use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Level {
    Undefined,
    High,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Connection {
    level: Level,
}

impl Connection {
    pub fn new() -> Connection {
        Connection { level: Level::Undefined }
    }

    pub fn level(&self) -> Level {
        self.level
    }

   pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::Undefined => write!(f, "Undefined"),
            Level::High => write!(f, "High"),
            Level::Low => write!(f, "Low"),
        }
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_connection() {
        let connection = Connection::new();
        assert_eq!(connection.level(), Level::Undefined);
    }
    
    #[test]
    fn test_set_high_level() {
        let mut connection = Connection::new();
        connection.set_level(Level::High);
        assert_eq!(connection.level(), Level::High);
    }

    #[test]
    fn test_set_low_level() {
        let mut connection = Connection::new();
        connection.set_level(Level::Low);
        assert_eq!(connection.level(), Level::Low);
    }

    #[test]
    fn test_set_undefined_level() {
        let mut connection = Connection::new();
        connection.set_level(Level::Undefined);
        assert_eq!(connection.level(), Level::Undefined);
    }

    #[test]
    fn test_display_level() {
        assert_eq!(format!("{}", Level::Undefined), "Undefined");
        assert_eq!(format!("{}", Level::High), "High");
        assert_eq!(format!("{}", Level::Low), "Low");
    }
}
