pub enum AddressingMode {
    IMP, IMM,
    ZP0, ZPX,
    ZPY, REL,
    ABS, ABX,
    ABY, IND,
    IZX, IZY
}

pub enum Instruction {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS, CLC,
    CLD, CLI, CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP,
    JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA, PLP, ROL, ROR, RTI,
    RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX, TAY, TSX, TXA, TXS, TYA,

    /// Capture for unimplemented opcodes.
    /// We will implement these later...
    XXX
}

#[derive(Clone, Copy)]
pub struct Opcode {
    instruction: Instruction,
    addressing_mode: AddressingMode,
    cycles: usize,
    raw_data: u8
}

impl Opcode {
    pub fn from(instruction: Instruction, addressing_mode: AddressingMode, cycles: usize, raw_data: u8) -> Opcode {
        Opcode {
            addressing_mode,
            instruction,
            cycles,
            raw_data
        }
    }
}