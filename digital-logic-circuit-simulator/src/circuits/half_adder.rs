use core::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use crate::components::{
    Connection, Component,
    gates::{ANDGate, XORGate},
};

pub struct HalfAdder {
    components: Vec<Box<dyn Component>>,
}

impl HalfAdder {
    pub fn new(
        input1: Rc<RefCell<Connection>>,
        input2: Rc<RefCell<Connection>>,
        sum: Rc<RefCell<Connection>>,
        carry: Rc<RefCell<Connection>>,
    ) -> HalfAdder {
        let and_gate = ANDGate::new(input1.clone(), input2.clone(), carry.clone());
        let xor_gate = XORGate::new(input1, input2, sum);

        HalfAdder {
            components: vec![Box::new(and_gate), Box::new(xor_gate)],
        }
    }
}

impl fmt::Display for HalfAdder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, component) in self.components.iter().enumerate() {
            writeln!(f, "Component {}:", i + 1)?;
            writeln!(f, "{}", component)?;
        }
        Ok(())
    }
}

impl Component for HalfAdder {
    fn update(&mut self) {
        for component in self.components.iter_mut() {
            component.update();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Level;
    use super::*;

    #[test]
    fn test_update_half_adder() {
        let input1 = Rc::new(RefCell::new(Connection::new()));
        let input2 = Rc::new(RefCell::new(Connection::new()));
        let sum = Rc::new(RefCell::new(Connection::new()));
        let carry = Rc::new(RefCell::new(Connection::new()));
        let mut half_adder = HalfAdder::new(input1.clone(), input2.clone(), sum.clone(), carry.clone());

        for (input1_level, input2_level, expected_sum, expected_carry) in vec![
            (Level::Low, Level::Low, Level::Low, Level::Low),
            (Level::Low, Level::High, Level::High, Level::Low),
            (Level::High, Level::Low, Level::High, Level::Low),
            (Level::High, Level::High, Level::Low, Level::High),
            (Level::Undefined, Level::Low, Level::Undefined, Level::Undefined),
            (Level::Low, Level::Undefined, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::High, Level::Undefined, Level::Undefined),
            (Level::High, Level::Undefined, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::Undefined, Level::Undefined, Level::Undefined),
        ] {
            input1.borrow_mut().set_level(input1_level);
            input2.borrow_mut().set_level(input2_level);
            half_adder.update();
            assert_eq!(sum.borrow().level(), expected_sum);
            assert_eq!(carry.borrow().level(), expected_carry);
        }
    }
}
