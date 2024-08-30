use emulator::{decoder::Decoder, fetcher::Fetcher};
use std::fs::File;

pub mod window;
mod emulator;
fn main() {
    let mut fetcher = Fetcher::new();
    fetcher.load_rom_mem("roms/Hello_World.ch8");
    let first_instruction = fetcher.fetch_instruction();
    assert_eq!(first_instruction, 0x1202);
    let decoder = Decoder::new();
    if let Some(instr) = decoder.simple_instruction_map.get(&0x00E0) {
        instr.execute();
    }
}
