use enumflags2::{bitflags, make_bitflags, BitFlags};

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
    Carry      = 0b00000001,
    
    /// Bit 1, the [zero bit](https://www.nesdev.org/wiki/Status_flags#Z:_Zero).
    ///
    /// After most instructions that have a value result, this flag will either be set or cleared based on whether or not that value is equal to zero.
    Zero       = 0b00000010,
    
    /// Bit 2, the [interrupt disable bit](https://www.nesdev.org/wiki/Status_flags#I:_Interrupt_Disable).
    ///
    /// When set, all interrupts except the NMI are inhibited.
    /// Can be set or cleared directly with SEI, CLI.
    /// Automatically set by the CPU when an IRQ is triggered, and restored to its previous state by RTI.
    /// If the /IRQ line is low (IRQ pending) when this flag is cleared, an interrupt will immediately be triggered.
    IrqDisable = 0b00000100,
    
    /// Bit 3, the [decimal bit](https://www.nesdev.org/wiki/Status_flags#D:_Decimal).
    ///
    /// On the NES, this flag has no effect.
    /// On the original 6502, this flag causes some arithmetic instructions to use binary-coded decimal representation to make base 10 calculations easier.
    /// Can be set or cleared directly with SED, CLD.
    Decimal    = 0b00001000,
    
    /// Bit 4.
    ///
    /// The first of two unused bits. This particular bit is also known as the "B flag."
    /// It represents a signal in the CPU controlling whether or not it was processing an interrupt when the flags were pushed.
    Unused0    = 0b00010000,
    
    /// Bit 5.
    ///
    /// The second of two unused bits. This bit should always be 1 when the flags are pushed.
    Unused1    = 0b00100000,
    
    /// Bit 6, the [overflow bit](https://www.nesdev.org/wiki/Status_flags#V:_Overflow).
    ///
    /// ADC and SBC will set this flag if the signed result would be invalid, necessary for making signed comparisons.\
    /// BIT will load bit 6 of the addressed value directly into the V flag.\
    /// Can be cleared directly with CLV. There is no corresponding set instruction.
    Overflow   = 0b01000000,
    
    /// Bit 7, the [negative bit](https://www.nesdev.org/wiki/Status_flags#N:_Negative).
    ///
    /// After most instructions that have a value result, this flag will contain bit 7 of that result.
    /// BIT will load bit 7 of the addressed value directly into the N flag.
    Negative   = 0b10000000
}

struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: BitFlags<Status>
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0,
            p: make_bitflags!(Status::{})
        }
    }
}