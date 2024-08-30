use super::memory::{Memory, MemoryDirection};
use std::fs::File;
pub struct Fetcher {
    pc: usize,
    memory: Memory,
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher {
            pc: 0,
            memory: Memory::new(),
        }
    }

    pub fn load_rom_mem(&mut self, rom: &str) {
        self.memory.load_rom(rom);
        ()
    }

    pub fn fetch_instruction(&mut self) -> u16 {
        let instruction: u16 = self.memory.read_word_at(self.pc);
        self.pc += 2;
        instruction
    }
}