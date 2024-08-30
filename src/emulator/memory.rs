use std::fs::read;

const MEMORY_SIZE: usize = 0xE00; // Ignore emulator memory

pub enum MemoryDirection {
    Read,
    Write
}
pub struct Memory {
    raw: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        let mem: Vec<u8> = vec![0; MEMORY_SIZE];
        Memory {
            raw: mem,
        }
    }

    pub fn access_word_at(&mut self, offset: usize, data: u16, direction: MemoryDirection) -> Result<u16, &str> {
        if offset > MEMORY_SIZE - 1 {
            return Err("Fatal memory error: accessing memory out of the bounds.\n");
        }
        let word: u16 = u16::from_be_bytes([self.raw[offset], self.raw[offset + 1]]);

        match direction {
            MemoryDirection::Read => Ok(word),
            MemoryDirection::Write => {
                self.raw[offset] = (data >> 8) as u8;
                self.raw[offset+1] = data as u8;
                Ok(0)
            },
        }
    }

    pub fn read_word_at(&mut self, offset: usize) -> u16 {
        self.access_word_at(offset, 0, MemoryDirection::Read)
            .expect("Fatal memory error: reading memory out of the bounds. Exiting.\n")
    }

    pub fn write_word_at(&mut self, offset: usize, data: u16) -> u16 {
        self.access_word_at(offset, data, MemoryDirection::Write)
            .expect("Fatal memory error: writing memory out of the bounds. Exiting.\n")
    }

    pub fn load_rom(&mut self, rom: &str) {
        let mut counter = 0;
        let prog = read(rom)
            .expect("Problem reading ROM file. Exiting.");

        for byte in prog.iter() {
            self.raw[counter] = *byte;
            counter += 1;
        }
    
        ()
    }

}