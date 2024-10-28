use std::rc::Rc;
use std::cell::RefCell;
use crate::components::{Connection, Level, BaseComponent, Component};

pub struct ANDGate {
    base: BaseComponent,
}

impl ANDGate {
    pub fn new(input1: Rc<RefCell<Connection>>, input2: Rc<RefCell<Connection>>, output: Rc<RefCell<Connection>>) -> ANDGate {
        let mut and_gate = ANDGate {
            base: BaseComponent::new("AND Gate"),
        };
        and_gate.base.add_input(input1);
        and_gate.base.add_input(input2);
        and_gate.base.add_output(output);
        and_gate
    }
}

impl Component for ANDGate {
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
            } else if input0_level == Level::High && input1_level == Level::High {
                output.set_level(Level::High);
            } else {
                output.set_level(Level::Low);
            }
        } else {
            println!("Output not found for AND Gate");
        }
    }
}

impl std::fmt::Display for ANDGate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.base.display(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_and_gate() {
        let input1 = Rc::new(RefCell::new(Connection::new()));
        let input2 = Rc::new(RefCell::new(Connection::new()));
        let output = Rc::new(RefCell::new(Connection::new()));
        let mut and_gate = ANDGate::new(input1.clone(), input2.clone(), output.clone());

        for (input1_level, input2_level, expected_output) in vec![
            (Level::Low, Level::Low, Level::Low),
            (Level::Low, Level::High, Level::Low),
            (Level::High, Level::Low, Level::Low),
            (Level::High, Level::High, Level::High),
            (Level::Undefined, Level::Low, Level::Undefined),
            (Level::Low, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::High, Level::Undefined),
            (Level::High, Level::Undefined, Level::Undefined),
            (Level::Undefined, Level::Undefined, Level::Undefined),
        ] {
            input1.borrow_mut().set_level(input1_level);
            input2.borrow_mut().set_level(input2_level);
            and_gate.update();
            assert_eq!(output.borrow().level(), expected_output);
        }
    }
}