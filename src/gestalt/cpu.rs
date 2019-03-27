mod instruction;
mod address_mode;

use instruction::Instruction;
use address_mode::AddressMode;

pub struct CPU {
    cycle: u8,
    register: Register,
    opcodes: [Option<(Instruction, AddressMode)>; 256]
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            cycle: 0,
            register: Register::new(),
            opcodes: generate_decoder()
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


fn generate_decoder() -> [Option<(Instruction, AddressMode)>; 256] {
    let mut decoder: [Option<(Instruction, AddressMode)>; 256] =  [None; 256];
    // break operation
    decoder[0x01] = Some((Instruction::BRK, AddressMode::Immediate));

    // ADC
    decoder[0x69] = Some((Instruction::ADC, AddressMode::Immediate));
    decoder[0x65] = Some((Instruction::ADC, AddressMode::ZeroPage));
    decoder[0x75] = Some((Instruction::ADC, AddressMode::ZeroPageX));
    decoder[0x6D] = Some((Instruction::ADC, AddressMode::Absolute));
    decoder[0x7D] = Some((Instruction::ADC, AddressMode::AbsoluteX));
    decoder[0x79] = Some((Instruction::ADC, AddressMode::AbsoluteY));
    decoder[0x61] = Some((Instruction::ADC, AddressMode::IndirectX));
    decoder[0x71] = Some((Instruction::ADC, AddressMode::IndirectY));

    // AND
    decoder[0x29] = Some((Instruction::AND, AddressMode::Immediate));
    decoder[0x25] = Some((Instruction::AND, AddressMode::ZeroPage));
    decoder[0x35] = Some((Instruction::AND, AddressMode::ZeroPageX));
    decoder[0x3D] = Some((Instruction::AND, AddressMode::Absolute));
    decoder[0x3D] = Some((Instruction::AND, AddressMode::AbsoluteX));
    decoder[0x39] = Some((Instruction::AND, AddressMode::AbsoluteY));
    decoder[0x21] = Some((Instruction::AND, AddressMode::IndirectX));
    decoder[0x31] = Some((Instruction::AND, AddressMode::IndirectY));

    // ASL
    decoder[0x0A] = Some((Instruction::ASL, AddressMode::Accumulator));
    decoder[0x06] = Some((Instruction::ASL, AddressMode::ZeroPage));
    decoder[0x16] = Some((Instruction::ASL, AddressMode::ZeroPageX));
    decoder[0x0E] = Some((Instruction::ASL, AddressMode::Absolute));
    decoder[0x1E] = Some((Instruction::ASL, AddressMode::AbsoluteX));

    // BCC
    decoder[0x90] = Some((Instruction::BCC, AddressMode::Relative));

    // BCS
    decoder[0xB0] = Some((Instruction::BCS, AddressMode::Relative));

    // BEQ
    decoder[0xF0] = Some((Instruction::BEQ, AddressMode::Relative));

    // BIT
    decoder[0x24] = Some((Instruction::BIT, AddressMode::ZeroPage));
    decoder[0x2C] = Some((Instruction::BIT, AddressMode::Absolute));

    // BMI
    decoder[0x30] = Some((Instruction::BMI, AddressMode::Relative));

    // BNE
    decoder[0xD0] = Some((Instruction::BNE, AddressMode::Relative));

    // BPL
    decoder[0x10] = Some((Instruction::BPL, AddressMode::Relative));

    // BVC
    decoder[0x50] = Some((Instruction::BVC, AddressMode::Relative));

    // BVS
    decoder[0x70] = Some((Instruction::BVS, AddressMode::Relative));

    // CLC
    decoder[0x18] = Some((Instruction::CLC, AddressMode::Implied));

    // CLD
    decoder[0xD8] = Some((Instruction::CLD, AddressMode::Implied));

    // CLI
    decoder[0x58] = Some((Instruction::CLI, AddressMode::Implied));

    // CLV
    decoder[0xB8] = Some((Instruction::CLV, AddressMode::Implied));

    // CMP
    decoder[0xC9] = Some((Instruction::CMP, AddressMode::Immediate));
    decoder[0xC5] = Some((Instruction::CMP, AddressMode::ZeroPage));
    decoder[0xD5] = Some((Instruction::CMP, AddressMode::ZeroPageX));
    decoder[0xCD] = Some((Instruction::CMP, AddressMode::Absolute));
    decoder[0xDD] = Some((Instruction::CMP, AddressMode::AbsoluteX));
    decoder[0xD9] = Some((Instruction::CMP, AddressMode::AbsoluteY));
    decoder[0xC1] = Some((Instruction::CMP, AddressMode::IndirectX));
    decoder[0xD1] = Some((Instruction::CMP, AddressMode::IndirectY));

    // CPX
    decoder[0xE0] = Some((Instruction::CPX, AddressMode::Immediate));
    decoder[0xE4] = Some((Instruction::CPX, AddressMode::ZeroPage));
    decoder[0xEC] = Some((Instruction::CPX, AddressMode::Absolute));

    // CPY
    decoder[0xC0] = Some((Instruction::CPY, AddressMode::Immediate));
    decoder[0xC4] = Some((Instruction::CPY, AddressMode::ZeroPage));
    decoder[0xCC] = Some((Instruction::CPY, AddressMode::Absolute));

    // DEC
    decoder[0xC6] = Some((Instruction::DEC, AddressMode::ZeroPage));
    decoder[0xD6] = Some((Instruction::DEC, AddressMode::ZeroPageX));
    decoder[0xCE] = Some((Instruction::DEC, AddressMode::Absolute));
    decoder[0xDE] = Some((Instruction::DEC, AddressMode::AbsoluteX));

    // DEX
    decoder[0xCA] = Some((Instruction::DEX, AddressMode::Implied));

    // DEY
    decoder[0x88] = Some((Instruction::DEY, AddressMode::Implied));

    // EOR
    decoder[0x49] = Some((Instruction::EOR, AddressMode::Immediate));
    decoder[0x45] = Some((Instruction::EOR, AddressMode::ZeroPage));
    decoder[0x55] = Some((Instruction::EOR, AddressMode::ZeroPageX));
    decoder[0x4D] = Some((Instruction::EOR, AddressMode::Absolute));
    decoder[0x5D] = Some((Instruction::EOR, AddressMode::AbsoluteX));
    decoder[0x59] = Some((Instruction::EOR, AddressMode::AbsoluteY));
    decoder[0x41] = Some((Instruction::EOR, AddressMode::IndirectX));
    decoder[0x51] = Some((Instruction::EOR, AddressMode::IndirectY));

    // INC
    decoder[0xE6] = Some((Instruction::INC, AddressMode::ZeroPage));
    decoder[0xF6] = Some((Instruction::INC, AddressMode::ZeroPageX));
    decoder[0xEE] = Some((Instruction::INC, AddressMode::Absolute));
    decoder[0xFE] = Some((Instruction::INC, AddressMode::AbsoluteX));

    // DEX
    decoder[0xE8] = Some((Instruction::INX, AddressMode::Implied));

    // DEY
    decoder[0xC8] = Some((Instruction::INY, AddressMode::Implied));

    // JMP
    decoder[0x4C] = Some((Instruction::JMP, AddressMode::Absolute));
    decoder[0x6C] = Some((Instruction::JMP, AddressMode::Indirect));

    // JSR
    decoder[0x20] = Some((Instruction::JSR, AddressMode::Absolute));

    // LDA
    decoder[0xA9] = Some((Instruction::LDA, AddressMode::Immediate));
    decoder[0xA5] = Some((Instruction::LDA, AddressMode::ZeroPage));
    decoder[0xB5] = Some((Instruction::LDA, AddressMode::ZeroPageX));
    decoder[0xAD] = Some((Instruction::LDA, AddressMode::Absolute));
    decoder[0xBD] = Some((Instruction::LDA, AddressMode::AbsoluteX));
    decoder[0xB9] = Some((Instruction::LDA, AddressMode::AbsoluteY));
    decoder[0xA1] = Some((Instruction::LDA, AddressMode::IndirectX));
    decoder[0xB1] = Some((Instruction::LDA, AddressMode::IndirectY));

    // LDX
    decoder[0xA2] = Some((Instruction::LDX, AddressMode::Immediate));
    decoder[0xA6] = Some((Instruction::LDX, AddressMode::ZeroPage));
    decoder[0xB6] = Some((Instruction::LDX, AddressMode::ZeroPageY));
    decoder[0xAE] = Some((Instruction::LDX, AddressMode::Absolute));
    decoder[0xBE] = Some((Instruction::LDX, AddressMode::AbsoluteY));


    // LDY
    decoder[0xA0] = Some((Instruction::LDY, AddressMode::Immediate));
    decoder[0xA4] = Some((Instruction::LDY, AddressMode::ZeroPage));
    decoder[0xB4] = Some((Instruction::LDY, AddressMode::ZeroPageX));
    decoder[0xAC] = Some((Instruction::LDY, AddressMode::Absolute));
    decoder[0xBC] = Some((Instruction::LDY, AddressMode::AbsoluteX));

    // LSR
    decoder[0x4A] = Some((Instruction::LSR, AddressMode::Accumulator));
    decoder[0x46] = Some((Instruction::LSR, AddressMode::ZeroPage));
    decoder[0x56] = Some((Instruction::LSR, AddressMode::ZeroPageX));
    decoder[0x4E] = Some((Instruction::LSR, AddressMode::Absolute));
    decoder[0x5E] = Some((Instruction::LSR, AddressMode::AbsoluteX));

    // NOP
    decoder[0xEA] = Some((Instruction::NOP, AddressMode::Implied));

    // ORA
    decoder[0x09] = Some((Instruction::ORA, AddressMode::Immediate));
    decoder[0x05] = Some((Instruction::ORA, AddressMode::ZeroPage));
    decoder[0x15] = Some((Instruction::ORA, AddressMode::ZeroPageX));
    decoder[0x0D] = Some((Instruction::ORA, AddressMode::Absolute));
    decoder[0x1D] = Some((Instruction::ORA, AddressMode::AbsoluteX));
    decoder[0x19] = Some((Instruction::ORA, AddressMode::AbsoluteY));
    decoder[0x01] = Some((Instruction::ORA, AddressMode::IndirectX));
    decoder[0x11] = Some((Instruction::ORA, AddressMode::IndirectY));

    // PHA
    decoder[0x48] = Some((Instruction::PHA, AddressMode::Implied));

    // PHP
    decoder[0x08] = Some((Instruction::PHP, AddressMode::Implied));

    // PLA
    decoder[0x68] = Some((Instruction::PLA, AddressMode::Implied));

    // PLP
    decoder[0x28] = Some((Instruction::PLP, AddressMode::Implied));

    // ROL
    decoder[0x4A] = Some((Instruction::ROL, AddressMode::Accumulator));
    decoder[0x46] = Some((Instruction::ROL, AddressMode::ZeroPage));
    decoder[0x56] = Some((Instruction::ROL, AddressMode::ZeroPageX));
    decoder[0x4E] = Some((Instruction::ROL, AddressMode::Absolute));
    decoder[0x5E] = Some((Instruction::ROL, AddressMode::AbsoluteX));

    // ROR
    decoder[0x6A] = Some((Instruction::ROR, AddressMode::Accumulator));
    decoder[0x66] = Some((Instruction::ROR, AddressMode::ZeroPage));
    decoder[0x76] = Some((Instruction::ROR, AddressMode::ZeroPageX));
    decoder[0x6E] = Some((Instruction::ROR, AddressMode::Absolute));
    decoder[0x7E] = Some((Instruction::ROR, AddressMode::AbsoluteX));

    // RTI
    decoder[0x40] = Some((Instruction::RTI, AddressMode::Implied));

    // RTS
    decoder[0x60] = Some((Instruction::RTS, AddressMode::Implied));

    // SBC
    decoder[0xE9] = Some((Instruction::SBC, AddressMode::Immediate));
    decoder[0xE5] = Some((Instruction::SBC, AddressMode::ZeroPage));
    decoder[0xF5] = Some((Instruction::SBC, AddressMode::ZeroPageX));
    decoder[0xED] = Some((Instruction::SBC, AddressMode::Absolute));
    decoder[0xFD] = Some((Instruction::SBC, AddressMode::AbsoluteX));
    decoder[0xF9] = Some((Instruction::SBC, AddressMode::AbsoluteY));
    decoder[0xE1] = Some((Instruction::SBC, AddressMode::IndirectX));
    decoder[0xF1] = Some((Instruction::SBC, AddressMode::IndirectY));

    // SEC
    decoder[0x38] = Some((Instruction::SEC, AddressMode::Implied));

    // SED
    decoder[0xF8] = Some((Instruction::SED, AddressMode::Implied));

    // SEI
    decoder[0x78] = Some((Instruction::SEI, AddressMode::Implied));

    // STA
    decoder[0x85] = Some((Instruction::STA, AddressMode::ZeroPage));
    decoder[0x95] = Some((Instruction::STA, AddressMode::ZeroPageX));
    decoder[0x8D] = Some((Instruction::STA, AddressMode::Absolute));
    decoder[0x9D] = Some((Instruction::STA, AddressMode::AbsoluteX));
    decoder[0x99] = Some((Instruction::STA, AddressMode::AbsoluteY));
    decoder[0x81] = Some((Instruction::STA, AddressMode::IndirectX));
    decoder[0x91] = Some((Instruction::STA, AddressMode::IndirectY));

    // LDX
    decoder[0x86] = Some((Instruction::STX, AddressMode::ZeroPage));
    decoder[0x96] = Some((Instruction::STX, AddressMode::ZeroPageY));
    decoder[0x8E] = Some((Instruction::STX, AddressMode::AbsoluteY));


    // LDY
    decoder[0x84] = Some((Instruction::STY, AddressMode::ZeroPage));
    decoder[0x94] = Some((Instruction::STY, AddressMode::ZeroPageX));
    decoder[0x8C] = Some((Instruction::STY, AddressMode::AbsoluteX));

    // TAX
    decoder[0xAA] = Some((Instruction::TAX, AddressMode::Implied));

    // TAY
    decoder[0xA8] = Some((Instruction::TAY, AddressMode::Implied));

    // TSX
    decoder[0xBA] = Some((Instruction::TSX, AddressMode::Implied));

    // TXA
    decoder[0x8A] = Some((Instruction::TXA, AddressMode::Implied));

    // TXS
    decoder[0x9A] = Some((Instruction::TXS, AddressMode::Implied));

    // TYA
    decoder[0x98] = Some((Instruction::TYA, AddressMode::Implied));


    decoder
}

