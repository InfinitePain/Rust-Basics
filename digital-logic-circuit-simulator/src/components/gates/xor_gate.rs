use std::rc::Rc;
use std::cell::RefCell;
use crate::components::{Connection, Level, BaseComponent, Component};

pub struct XORGate {
    base: BaseComponent,
}

impl XORGate {
    pub fn new(input1: Rc<RefCell<Connection>>, input2: Rc<RefCell<Connection>>, output: Rc<RefCell<Connection>>) -> XORGate {
        let mut xor_gate = XORGate {
            base: BaseComponent::new("XOR Gate"),
        };
        xor_gate.base.add_input(input1);
        xor_gate.base.add_input(input2);
        xor_gate.base.add_output(output);
        xor_gate
    }
}

impl Component for XORGate {
    fn update(&mut self) {
        let input0_level = self
            .base
            .get_input(0)
            .map_or(Level::Undefined, |c| c.borrow().level());

        let input1_level = self
            .base
            .get_input(1)
            .map_or(Level::Undefined, |c| c.borrow().level());

        if let Some(output) = self.base.get_output(0) {
            let mut output = output.borrow_mut();
            if input0_level == Level::Undefined || input1_level == Level::Undefined {
                output.set_level(Level::Undefined);
            } else if input0_level != input1_level {
                output.set_level(Level::High);
            } else {
                output.set_level(Level::Low);
            }
        } else {
            println!("Output not found for XOR Gate");
        }
    }
}

impl std::fmt::Display for XORGate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.base.display(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_xor_gate() {
        let input1 = Rc::new(RefCell::new(Connection::new()));
        let input2 = Rc::new(RefCell::new(Connection::new()));
        let output = Rc::new(RefCell::new(Connection::new()));
        let mut xor_gate = XORGate::new(input1.clone(), input2.clone(), output.clone());

        for (input1_level, input2_level, expected_output) in vec![
            (Level::Low, Level::Low, Level::Low),
            (Level::Low, Level::High, Level::High),
            (Level::High, Level::Low, Level::High),
            (Level::High, Level::High, Level::Low),
            (Level::Undefined, Level::Low, Level::Undefined),
            (Level::Low, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::High, Level::Undefined),
            (Level::High, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::Undefined, Level::Undefined),
        ] {
            input1.borrow_mut().set_level(input1_level);
            input2.borrow_mut().set_level(input2_level);
            xor_gate.update();
            assert_eq!(output.borrow().level(), expected_output);
        }
    }
}