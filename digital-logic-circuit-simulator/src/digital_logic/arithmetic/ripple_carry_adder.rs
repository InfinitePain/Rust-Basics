use std::cell::RefCell;
use std::rc::Rc;

use crate::components::{Component, Connection, Level};
use crate::circuits::{FullAdder, HalfAdder};

pub struct RippleCarryAdder {
    n_bit: usize,
    input1: Vec<Rc<RefCell<Connection>>>,
    input2: Vec<Rc<RefCell<Connection>>>,
    sum: Vec<Rc<RefCell<Connection>>>,
    carry: Vec<Rc<RefCell<Connection>>>,
    half_adder: HalfAdder,
    full_adders: Vec<FullAdder>,
}

impl RippleCarryAdder {
    pub fn new(n_bit: usize) -> Self {
        let input1 : Vec<Rc<RefCell<Connection>>> = (0..n_bit)
            .map(|_| Rc::new(RefCell::new(Connection::new())))
            .collect();
        let input2 : Vec<Rc<RefCell<Connection>>> = (0..n_bit)
            .map(|_| Rc::new(RefCell::new(Connection::new())))
            .collect();
        let sum: Vec<_> = (0..n_bit)
            .map(|_| Rc::new(RefCell::new(Connection::new())))
            .collect();
        let carry: Vec<_> = (0..n_bit)
            .map(|_| Rc::new(RefCell::new(Connection::new())))
            .collect();

        let half_adder = HalfAdder::new(
            Rc::clone(&input1[0]),
            Rc::clone(&input2[0]),
            Rc::clone(&sum[0]),
            Rc::clone(&carry[0]),
        );

        // Create full adders for remaining bits
        let mut full_adders = Vec::new();
        for i in 1..n_bit {
            let full_adder = FullAdder::new(
                Rc::clone(&input1[i]),
                Rc::clone(&input2[i]),
                Rc::clone(&carry[i - 1]),
                Rc::clone(&sum[i]),
                Rc::clone(&carry[i]),
            );
            full_adders.push(full_adder);
        }

        Self {
            n_bit,
            input1,
            input2,
            sum,
            carry,
            half_adder,
            full_adders,
        }
    }

    fn string_to_connections(&self, binary_str: &str) -> Vec<Rc<RefCell<Connection>>> {
        binary_str
            .chars()
            .rev()
            .map(|c| {
                let conn = Rc::new(RefCell::new(Connection::new()));
                conn.borrow_mut().set_level(if c == '1' { Level::High } else { Level::Low });
                conn
            })
            .collect()
    }

    fn connections_to_string(&self, connections: &[Rc<RefCell<Connection>>]) -> String {
        connections
            .iter()
            .rev()
            .map(|conn| if conn.borrow().level() == Level::High { '1' } else { '0' })
            .collect()
    }

    pub fn calculate(&mut self, a: &str, b: &str) -> (String, bool) {
        let input1 = self.string_to_connections(&format!("{:0>width$}", a, width = self.n_bit));
        let input2 = self.string_to_connections(&format!("{:0>width$}", b, width = self.n_bit));
        for i in 0..self.n_bit {
            self.input1[i].borrow_mut().set_level(input1[i].borrow().level());
            self.input2[i].borrow_mut().set_level(input2[i].borrow().level());
        }

        self.half_adder.update();
        for adder in self.full_adders.iter_mut() {
            adder.update();
        }


        // Collect the sum as a string and determine overflow
        let sum_str = self.connections_to_string(&self.sum);
        let overflow = self.carry.last().unwrap().borrow().level() == Level::High;

        (sum_str, overflow)
    }
}
