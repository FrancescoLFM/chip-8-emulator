use super::{decoder::{Decoder, Instruction}, fetcher::Fetcher, memory::Memory};

const REG_NUM: usize = 16;
const STACK_SIZE: usize = 16;

pub struct Cpu {
    fetcher: Fetcher,
    decoder: Decoder,
    addr_reg: u16,
    delay_reg: u8,
    sound_reg: u8,
    stack_reg: u8,
    stack_mem: [u16; STACK_SIZE],
    reg_file: [u8; REG_NUM],
}

impl Cpu{
    pub fn new(ram: Memory ) -> Self {
        Cpu {
            fetcher: Fetcher::new(ram),
            decoder: Decoder::new(),
            addr_reg: 0,
            delay_reg: 0,
            sound_reg: 0,
            stack_reg: 0,
            stack_mem: [0; STACK_SIZE],
            reg_file: [0; REG_NUM],
        }
    }

    pub fn load_rom(&mut self, rom: &str) {
        self.fetcher.load_rom_mem(rom);
    }

    pub fn register_set(&mut self, reg: usize, data: u8) {
        if reg >= REG_NUM {
            panic!("Invalid write register: V{:x}", reg); 
        }
        self.reg_file[reg] = data;
    }

    pub fn register_get(&mut self, reg: usize) -> u8 {
        if reg >= REG_NUM {
            panic!("Invalid read register: V{:x}", reg); 
        }
        self.reg_file[reg]
    }

    pub fn jmp(&mut self, addr: u16) {
        self.fetcher.pc_set(addr as usize);
    }

    pub fn start_rom(&mut self) {
        loop {
            let opcode: u16 = self.fetcher.fetch_instruction();
            let instr: &Instruction = self.decoder.decode_opcode(opcode).unwrap();
            instr(opcode, self);
        }
    }
}