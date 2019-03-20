mod cpu;
mod ppu;
mod apu;

pub use self::cpu::CPU;
pub use self::ppu::PPU;
pub use self::apu::APU;

use std::error::Error;

pub struct Gestalt {
    cpu: CPU,
    ppu: PPU,
    apu: APU
}

impl Gestalt {
    pub fn new() -> Gestalt {
        Gestalt {
            cpu: CPU::new(),
            ppu: PPU::new(),
            apu: APU::new()
        }
    }

    pub fn emulate(&self) -> Result<(), Box<Error>> {
        Ok(println!("Emulation successfully ended"))
    }
}

