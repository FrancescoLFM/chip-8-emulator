use std::collections::HashMap;

pub trait SimpleInstruction {
    fn execute(&self);
}

#[derive(Default)]

struct Cls;

impl SimpleInstruction for Cls {
    fn execute(&self) {
        println!("Clearing the screen");
    }
}

pub struct Decoder {
    pub simple_instruction_map: HashMap<u16, Box<dyn SimpleInstruction>>,
}

impl Decoder {
    pub fn new() -> Self {
        Decoder {
            simple_instruction_map: HashMap::from([
                (0x00E0, Box::new(Cls) as Box<dyn SimpleInstruction>),
            ]
            ),
        }
    }
}