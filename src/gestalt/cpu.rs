mod instruction;

use instruction::Instruction;

pub struct CPU {
    cycle: u8,
    register: Register
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            cycle: 0,
            register: Register::new()
        }
    }
}

pub struct Register {
    program_counter: u16,
    stack_pointer: u8,
    accumulator: u8,
    x: u8,
    y: u8,
    status: u8
}

impl Register {
    pub fn new() -> Register {
        Register {
            program_counter: 0,
            stack_pointer: 0,
            accumulator: 0,
            x: 0,
            y: 0,
            status: 0
        }
    }
}

