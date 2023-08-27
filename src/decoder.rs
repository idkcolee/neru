use crate::instruction::{Opcode, Instruction, AddressingMode};

const DECODE_TABLE: Vec<Opcode> = vec![
    Opcode::from(Instruction::BRK, AddressingMode::IMP, 7, 0x00), Opcode::from(Instruction::ORA, AddressingMode::IZX, 6, 0x01), Opcode::from(Instruction::XXX, AddressingMode::IMP, 2, 0x02)
];