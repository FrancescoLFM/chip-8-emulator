use emulator::decoder::Decoder;

pub mod window;
pub mod emulator {
    pub mod decoder;
}

fn main() {
    let decoder = Decoder::new();
    if let Some(instr) = decoder.simple_instruction_map.get(&0x00E0) {
        instr.execute();
    }
}
