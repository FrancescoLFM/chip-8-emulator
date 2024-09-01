use super::memory::Memory;
pub struct Fetcher {
    pc: usize,
    memory: Memory,
}

impl Fetcher {
    pub fn new(mem: Memory) -> Self {
        Fetcher {
            pc: 0,
            memory: mem,
        }
    }

    pub fn load_rom_mem(&mut self, rom: &str) {
        self.memory.load_rom(rom);
        ()
    }

    pub fn pc_set(&mut self, addr: usize) {
        self.pc = addr;
    }

    pub fn fetch_instruction(&mut self) -> u16 {
        let instruction: u16 = self.memory.read_word_at(self.pc);
        self.pc += 2;
        instruction
    }
}