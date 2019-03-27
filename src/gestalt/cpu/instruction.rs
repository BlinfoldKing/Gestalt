#[derive(Copy, Clone)]
pub enum Instruction {
    // load and store operation
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,

    // register transfer
    TAX,
    TAY,
    TXA,
    TYA,

    // stack operation
    TSX,
    TXY,
    PHA,
    PHP,
    PLA,
    PLP,

    // logical operation
    AND,
    EOR,
    ORA,
    BIT,

    // arithmetic operation
    ADC,
    SBC,
    CMP,
    CPX,
    CPY,

    // inc and dec operation
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,

    // shifts operation
    ASL,
    LSR,
    ROL,
    ROR,

    // jump operation
    JMP,
    JSR,
    RTS,

    // branch operation
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,

    // status flag changes
    CLC,
    CLD,
    CLI,
    CLV,
    SEC,
    SED,
    SEI,

    // system function
    BRK,
    NOP,
    RTI
}

