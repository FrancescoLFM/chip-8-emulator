use emulator::{decoder::{Decoder, Instruction}, fetcher::Fetcher, memory::Memory};
use std::fs::File;

pub mod window;
mod emulator;
fn main() {
    let mut fetcher = Fetcher::new(Memory::new());
    fetcher.load_rom_mem("roms/Hello_World.ch8");
    let first_instruction = fetcher.fetch_instruction();
    assert_eq!(first_instruction, 0x1202);
    let decoder = Decoder::new();
    let instr: &Instruction = decoder.decode_opcode(first_instruction).unwrap();
    instr(first_instruction);
}
