use std::cmp::PartialEq;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let direction = match self.direction {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        };
        write!(f, "Robot at ({}, {}) facing {}", self.x, self.y, direction)
    }
}

impl Robot {
    fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            direction: Direction::Up,
        }
    }
    
    fn advance(&mut self) {
        self.direction = match self.direction {
            Direction::Up => {
                self.y += 1;
                Direction::Up
            }
            Direction::Down => {
                self.y -= 1;
                Direction::Down
            }
            Direction::Left => {
                self.x -= 1;
                Direction::Left
            }
            Direction::Right => {
                self.x += 1;
                Direction::Right
            }
        };
    }
    
    fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => {
                self.direction = Direction::Right;
            }
            Direction::Down => {
                self.direction = Direction::Left;
            }
            Direction::Left => {
                self.direction = Direction::Up;
            }
            Direction::Right => {
                self.direction = Direction::Down;
            }
        }
    }
    
    fn turn_left(&mut self) {
        match self.direction {
            Direction::Up => {
                self.direction = Direction::Left;
            }
            Direction::Down => {
                self.direction = Direction::Right;
            }
            Direction::Left => {
                self.direction = Direction::Down;
            }
            Direction::Right => {
                self.direction = Direction::Up;
            }
        }
    }
    
    fn give_command(&mut self, command: String) {
        // return error if empty
        if command.is_empty() {
            println!("Error: No command given");
            return;
        }
        
        // iterate over each character in the command
        for c in command.chars() {
            match c {
                'A' => self.advance(),
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                _ => println!("Error: Unknown command '{}'", c),
            }
        }
    }
}

// test the robot
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_robot() {
        let mut robot = Robot::new();
        robot.give_command("AALALALAA".to_string());
        assert_eq!(robot.x, 1);
        assert_eq!(robot.y, 1);
        assert_eq!(robot.direction, Direction::Left);
        
        let mut robot = Robot::new();
        robot.give_command("AARARARAA".to_string());
        assert_eq!(robot.x, 1);
        assert_eq!(robot.y, 1);
        assert_eq!(robot.direction, Direction::Right);
        
        let mut robot = Robot::new();
        robot.give_command("AALAARAA".to_string());
        assert_eq!(robot.x, -2);
        assert_eq!(robot.y, 4);
        assert_eq!(robot.direction, Direction::Up);
    }
}

fn main() {
    // List of command
    let commands = vec![
        "AALALALAA".to_string(), // Should print "Robot at (1, 1) facing left"
        "AARARARAA".to_string(), // Should print "Robot at (1, 1) facing right"
        "AALAARAA".to_string(), // Should print "Robot at (-2, 4) facing up"
    ];
    
    for command in commands {
        let mut robot = Robot::new();
        robot.give_command(command);
        println!("{}", robot);
    }
    
}
