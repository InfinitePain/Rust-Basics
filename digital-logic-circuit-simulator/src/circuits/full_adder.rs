use core::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use crate::components::{
    Connection, Component,
    gates::ORGate,
};
use crate::circuits::HalfAdder;

pub struct FullAdder {
    components: Vec<Box<dyn Component>>,
}

impl FullAdder {
    pub fn new(
        input1: Rc<RefCell<Connection>>,
        input2: Rc<RefCell<Connection>>,
        carry_in: Rc<RefCell<Connection>>,
        sum: Rc<RefCell<Connection>>,
        carry_out: Rc<RefCell<Connection>>,
    ) -> FullAdder {
        let half_adder1_sum = Rc::new(RefCell::new(Connection::new()));
        let half_adder1_carry = Rc::new(RefCell::new(Connection::new()));
        let half_adder1 = HalfAdder::new(input1.clone(), input2.clone(), half_adder1_sum.clone(), half_adder1_carry.clone());

        let half_adder2_sum = sum.clone();
        let half_adder2_carry = Rc::new(RefCell::new(Connection::new()));
        let half_adder2 = HalfAdder::new(half_adder1_sum.clone(), carry_in.clone(), half_adder2_sum.clone(), half_adder2_carry.clone());

        let or_gate = ORGate::new(half_adder1_carry.clone(), half_adder2_carry.clone(), carry_out.clone());

        FullAdder {
            components: vec![Box::new(half_adder1), Box::new(half_adder2), Box::new(or_gate)],
        }
    }
}

impl fmt::Display for FullAdder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, component) in self.components.iter().enumerate() {
            writeln!(f, "Component {}:", i + 1)?;
            writeln!(f, "{}", component)?;
        }
        Ok(())
    }
}

impl Component for FullAdder {
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
    fn test_update_full_adder() {
        let input1 = Rc::new(RefCell::new(Connection::new()));
        let input2 = Rc::new(RefCell::new(Connection::new()));
        let carry_in = Rc::new(RefCell::new(Connection::new()));
        let sum = Rc::new(RefCell::new(Connection::new()));
        let carry_out = Rc::new(RefCell::new(Connection::new()));
        let mut full_adder = FullAdder::new(input1.clone(), input2.clone(), carry_in.clone(), sum.clone(), carry_out.clone());

        for (input1_level, input2_level, carry_in_level, expected_sum, expected_carry_out) in vec![
            (Level::Low, Level::Low, Level::Low, Level::Low, Level::Low),
            (Level::Low, Level::Low, Level::High, Level::High, Level::Low),
            (Level::Low, Level::High, Level::Low, Level::High, Level::Low),
            (Level::Low, Level::High, Level::High, Level::Low, Level::High),
            (Level::High, Level::Low, Level::Low, Level::High, Level::Low),
            (Level::High, Level::Low, Level::High, Level::Low, Level::High),
            (Level::High, Level::High, Level::Low, Level::Low, Level::High),
            (Level::High, Level::High, Level::High, Level::High, Level::High),
            (Level::Undefined, Level::Low, Level::Low, Level::Undefined, Level::Undefined),
            (Level::Low, Level::Undefined, Level::Low, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::High, Level::Low, Level::Undefined, Level::Undefined),
            (Level::High, Level::Undefined, Level::Low, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::Low, Level::High, Level::Undefined, Level::Undefined),
            (Level::Low, Level::Undefined, Level::High, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::High, Level::High, Level::Undefined, Level::Undefined),
            (Level::High, Level::Undefined, Level::High, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::Undefined, Level::Low, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::Undefined, Level::High, Level::Undefined, Level::Undefined),
            (Level::Low, Level::Low, Level::Undefined, Level::Undefined, Level::Undefined),
            (Level::Low, Level::High, Level::Undefined, Level::Undefined, Level::Undefined),
            (Level::High, Level::Low, Level::Undefined, Level::Undefined, Level::Undefined),
            (Level::High, Level::High, Level::Undefined, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::Low, Level::Undefined, Level::Undefined, Level::Undefined),
            (Level::Low, Level::Undefined, Level::Undefined, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::High, Level::Undefined, Level::Undefined, Level::Undefined),
            (Level::High, Level::Undefined, Level::Undefined, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::Undefined, Level::Undefined, Level::Undefined, Level::Undefined),
        ] {
            input1.borrow_mut().set_level(input1_level);
            input2.borrow_mut().set_level(input2_level);
            carry_in.borrow_mut().set_level(carry_in_level);
            full_adder.update();
            assert_eq!(sum.borrow().level(), expected_sum);
            assert_eq!(carry_out.borrow().level(), expected_carry_out);
        }
    }
}