use emulator::{cpu::Cpu, memory::Memory};

pub mod window;
mod emulator;
fn main() {
    let ram = Memory::new();
    let mut cpu = Cpu::new(ram);
    cpu.load_rom("roms/Hello_World.ch8");
    cpu.start_rom();
}
