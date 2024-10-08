use std::collections::HashMap;

const OPCODE_MASK: u16 = 0xF000; 
const ADDRESS_MASK: u16 = 0x0FFF;

fn cls(_code: u16) {
    println!("Clearing the screen");
}

fn jmp(code: u16) {
    println!("Jumping to 0x{:x}", code & ADDRESS_MASK);
}

pub type Instruction = fn(u16);

pub struct Decoder {
    instruction_map: HashMap<u16, Instruction>,
}

impl Decoder {
    pub fn new() -> Self {
        Decoder {
            instruction_map: HashMap::from([
                (0x00E0, cls as Instruction),
                (0x1000, jmp)
            ]
            ),
        }
    }

    pub fn decode_opcode(&self, opcode: u16) -> Result<&Instruction, &str> {
        let mut _opcode = opcode;
        
        if opcode & OPCODE_MASK != 0 {
            _opcode = opcode & OPCODE_MASK;
        }
        if let Some(instr) = self.instruction_map.get(&_opcode) {
            return Ok(instr);
        }

        Err("Invalid opcode.")
    }
}