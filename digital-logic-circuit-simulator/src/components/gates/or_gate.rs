use core::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use crate::components::{Connection, Level, BaseComponent, Component};

pub struct ORGate {
    base: BaseComponent,
}

impl ORGate {
    pub fn new(input1: Rc<RefCell<Connection>>, input2: Rc<RefCell<Connection>>, output: Rc<RefCell<Connection>>) -> ORGate {
        let mut or_gate = ORGate {
            base: BaseComponent::new("OR Gate"),
        };
        or_gate.base.add_input(input1);
        or_gate.base.add_input(input2);
        or_gate.base.add_output(output);
        or_gate
    }
}

impl Component for ORGate {
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
            } else if input0_level == Level::High || input1_level == Level::High {
                output.set_level(Level::High);
            } else {
                output.set_level(Level::Low);
            }
        } else {
            println!("Output not found for OR Gate");
        }
    }
}

impl fmt::Display for ORGate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.base.display(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_or_gate() {
        let input1 = Rc::new(RefCell::new(Connection::new()));
        let input2 = Rc::new(RefCell::new(Connection::new()));
        let output = Rc::new(RefCell::new(Connection::new()));
        let mut or_gate = ORGate::new(input1.clone(), input2.clone(), output.clone());

        for (input1_level, input2_level, expected_output) in vec![
            (Level::Low, Level::Low, Level::Low),
            (Level::Low, Level::High, Level::High),
            (Level::High, Level::Low, Level::High),
            (Level::High, Level::High, Level::High),
            (Level::Undefined, Level::Low, Level::Undefined),
            (Level::Low, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::High, Level::Undefined),
            (Level::High, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::Undefined, Level::Undefined),
        ] {
            input1.borrow_mut().set_level(input1_level);
            input2.borrow_mut().set_level(input2_level);
            or_gate.update();
            assert_eq!(output.borrow().level(), expected_output);
        }
    }
}