use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use crate::components::Connection;

pub trait Component : fmt::Display {
    fn update(&mut self);
}

#[derive(PartialEq)]
pub struct BaseComponent {
    name: String,
    inputs: Vec<Rc<RefCell<Connection>>>,
    outputs: Vec<Rc<RefCell<Connection>>>,
}

impl BaseComponent {
    pub fn new(name: &str) -> BaseComponent {
        BaseComponent {
            name: String::from(name),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }


    pub fn add_input(&mut self, input: Rc<RefCell<Connection>>) {
        self.inputs.push(input);
    }

    pub fn add_output(&mut self, output: Rc<RefCell<Connection>>) {
        self.outputs.push(output);
    }
    
    pub fn get_input(&self, index: usize) -> Option<Rc<RefCell<Connection>>> {
        self.inputs.get(index).cloned()
    }

    pub fn get_output(&self, index: usize) -> Option<Rc<RefCell<Connection>>> {
        self.outputs.get(index).cloned()
    }
    
    pub fn display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Component: {}", self.name)?;
        for (i, input) in self.inputs.iter().enumerate() {
            writeln!(f, "Input {}: Level = {}", i, input.borrow().level())?;
        }
        for (i, output) in self.outputs.iter().enumerate() {
            writeln!(f, "Output {}: Level = {}", i, output.borrow().level())?;
        }
        Ok(())
    }
}

impl fmt::Display for BaseComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_base_component() {
        let base_component = BaseComponent::new("Base Component");
        assert_eq!(base_component.name, "Base Component");
        assert_eq!(base_component.inputs.len(), 0);
        assert_eq!(base_component.outputs.len(), 0);
    }
    
    #[test]
    fn test_add_input() {
        let mut base_component = BaseComponent::new("Base Component");
        let connection = Rc::new(RefCell::new(Connection::new()));
        base_component.add_input(connection.clone());
        assert_eq!(base_component.inputs.len(), 1);
        assert_eq!(base_component.inputs[0], connection);
    }
    
    #[test]
    fn test_add_output() {
        let mut base_component = BaseComponent::new("Base Component");
        let connection = Rc::new(RefCell::new(Connection::new()));
        base_component.add_output(connection.clone());
        assert_eq!(base_component.outputs.len(), 1);
        assert_eq!(base_component.outputs[0], connection);
    }
    
    #[test]
    fn test_get_input() {
        let mut base_component = BaseComponent::new("Base Component");
        let connection = Rc::new(RefCell::new(Connection::new()));
        base_component.add_input(connection.clone());
        assert_eq!(base_component.get_input(0).unwrap(), connection);
    }
    
    #[test]
    fn test_get_output() {
        let mut base_component = BaseComponent::new("Base Component");
        let connection = Rc::new(RefCell::new(Connection::new()));
        base_component.add_output(connection.clone());
        assert_eq!(base_component.get_output(0).unwrap(), connection);
    }
}
