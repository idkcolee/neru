use std::ops::Add;
use enumflags2::{bitflags, make_bitflags, BitFlags};
use crate::cpu::Status::I;
use crate::instruction::AddressingMode;
use crate::memory::Memory;

/// Used for the bitflags that represent the 6502's [flags register](https://www.nesdev.org/wiki/Status_flags) (P).
/// It is composed of six one-bit registers but is architecturally byte-wide.
///
/// # Structure
/// ```ignore
/// 7  bit  0
/// ---- ----
/// NVss DIZC
/// |||| ||||
/// |||| |||+- Carry
/// |||| ||+-- Zero
/// |||| |+--- Interrupt Disable
/// |||| +---- Decimal
/// ||++------ No CPU effect, see: the B flag
/// |+-------- Overflow
/// +--------- Negative
/// ```
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    /// Bit 0, the [carry bit](https://www.nesdev.org/wiki/Status_flags#C:_Carry).
    ///
    /// # Behaviors
    /// - After ADC, this is the carry result of the addition.
    /// - After SBC or CMP, this flag will be set if no borrow was the result, or alternatively a "greater than or equal" result.
    /// - After a shift instruction (ASL, LSR, ROL, ROR), this contains the bit that was shifted out.
    /// - Increment and decrement instructions do not affect the carry flag.
    /// - Can be set or cleared directly with SEC, CLC.
    C = 0b00000001,
    
    /// Bit 1, the [zero bit](https://www.nesdev.org/wiki/Status_flags#Z:_Zero).
    ///
    /// # Behaviors
    /// - After most instructions that have a value result, this flag will either be set or cleared based on whether or not that value is equal to zero.
    Z = 0b00000010,
    
    /// Bit 2, the [interrupt disable bit](https://www.nesdev.org/wiki/Status_flags#I:_Interrupt_Disable).
    ///
    /// # Behaviors
    /// - When set, all interrupts except the NMI are inhibited.
    /// - Can be set or cleared directly with SEI, CLI.
    /// - Automatically set by the CPU when an IRQ is triggered, and restored to its previous state by RTI.
    /// - If the /IRQ line is low (IRQ pending) when this flag is cleared, an interrupt will immediately be triggered.
    I = 0b00000100,
    
    /// Bit 3, the [decimal bit](https://www.nesdev.org/wiki/Status_flags#D:_Decimal).
    ///
    /// # Behaviors
    /// - On the NES, this flag has no effect.
    /// - On the original 6502, this flag causes some arithmetic instructions to use binary-coded decimal representation to make base 10 calculations easier.
    /// - Can be set or cleared directly with SED, CLD.
    D = 0b00001000,
    
    /// Bit 4.
    ///
    /// # Behaviors
    /// - The first of two unused bits. This particular bit is also known as the "B flag."
    /// - It represents a signal in the CPU controlling whether or not it was processing an interrupt when the flags were pushed.
    B = 0b00010000,
    
    /// Bit 5.
    ///
    /// # Behaviors
    /// - The second of two unused bits. This bit should always be 1 when the flags are pushed.
    Unused = 0b00100000,
    
    /// Bit 6, the [overflow bit](https://www.nesdev.org/wiki/Status_flags#V:_Overflow).
    ///
    /// # Behaviors
    /// - ADC and SBC will set this flag if the signed result would be invalid, necessary for making signed comparisons.
    /// - BIT will load bit 6 of the addressed value directly into the V flag.
    /// - Can be cleared directly with CLV. There is no corresponding set instruction.
    V = 0b01000000,
    
    /// Bit 7, the [negative bit](https://www.nesdev.org/wiki/Status_flags#N:_Negative).
    ///
    /// # Behaviors
    /// - After most instructions that have a value result, this flag will contain bit 7 of that result.
    /// - BIT will load bit 7 of the addressed value directly into the N flag.
    N = 0b10000000
}

/// Represents the internal state of the 6502.
pub struct Cpu {
    /// Accumulator register
    a: u8,
    /// Index register X
    x: u8,
    /// Index register Y
    y: u8,
    /// Stack pointer
    sp: u8,
    /// Program counter
    pc: u16,
    /// Status register
    p: BitFlags<Status>,
    /// Memory handler
    memory: Memory
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            p: make_bitflags!(Status::{}),
            memory: Memory::new()
        }
    }

    pub fn execute() {
        // 1. read byte @ pc
        // 2. opcode[byte] -> addressingmode, cycles
        // 3. read 0-2 more bytes
        // 4. execute
        // 5. wait, count cycles, complete
    }

    fn get_operand_address(&mut self, addressing_mode: AddressingMode) -> u16 {
        match addressing_mode {
            AddressingMode::IMP => 0,
            AddressingMode::IMM => self.pc,
            AddressingMode::ZPG => self.memory.read(self.pc) as u16,
            AddressingMode::ZPX => {},
            AddressingMode::ZPY => {},
            AddressingMode::REL => {},
            AddressingMode::ABS => self.memory.read(self.pc),
            AddressingMode::ABX => {},
            AddressingMode::ABY => {},
            AddressingMode::IND => {},
            AddressingMode::IDX => {},
            AddressingMode::IDY => {},
        }
    }
}