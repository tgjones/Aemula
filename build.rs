use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(PartialEq)]
enum AddressingMode {
    None,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndexedIndirectX,
    IndirectIndexedY,
    Indirect,
    JSR,
    Invalid,
}

impl AddressingMode {
    fn as_string(&self) -> &'static str {
        match self {
            AddressingMode::None => "",
            AddressingMode::Accumulator => "",
            AddressingMode::Immediate => "#",
            AddressingMode::ZeroPage => "zp",
            AddressingMode::ZeroPageX => "zp,X",
            AddressingMode::ZeroPageY => "zp,Y",
            AddressingMode::Absolute => "abs",
            AddressingMode::AbsoluteX => "abs,X",
            AddressingMode::AbsoluteY => "abs,Y",
            AddressingMode::IndexedIndirectX => "(zp,X)",
            AddressingMode::IndirectIndexedY => "(zp),Y",
            AddressingMode::Indirect => "ind",
            AddressingMode::JSR => "",
            AddressingMode::Invalid => "invalid",
        }
    }
}

#[derive(PartialEq)]
enum MemoryAccess {
    None,
    Read,
    Write,
    ReadWrite,
}

struct Instruction(u8, &'static str, AddressingMode, MemoryAccess);

static INSTRUCTIONS: [Instruction; 239] = [
    // Interrupt, jump, subroutine
    Instruction(0x00, "BRK", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x20, "JSR", AddressingMode::JSR,              MemoryAccess::None),
    Instruction(0x40, "RTI", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x60, "RTS", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x4C, "JMP", AddressingMode::Absolute,         MemoryAccess::None),
    Instruction(0x6C, "JMP", AddressingMode::Indirect,         MemoryAccess::None),

    // Flags
    Instruction(0x18, "CLC", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x38, "SLC", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x58, "CLI", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x78, "SEI", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xB8, "CLV", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xD8, "CLD", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xF8, "SED", AddressingMode::None,             MemoryAccess::None),

    // Branch
    Instruction(0x10, "BPL", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0x30, "BMI", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0x50, "BVC", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0x70, "BVS", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0x90, "BCC", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0xB0, "BCS", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0xD0, "BNE", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0xF0, "BEQ", AddressingMode::Immediate,        MemoryAccess::None),

    // Stack
    Instruction(0x08, "PHP", AddressingMode::None,             MemoryAccess::Write),
    Instruction(0x28, "PLP", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x48, "PHA", AddressingMode::None,             MemoryAccess::Write),
    Instruction(0x68, "PLA", AddressingMode::None,             MemoryAccess::None),

    // Implied arithmetic
    Instruction(0x88, "DEY", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xCA, "DEX", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xC8, "INY", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xE8, "INX", AddressingMode::None,             MemoryAccess::None),

    // Transfer
    Instruction(0x8A, "TXA", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x9A, "TXS", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x98, "TYA", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xA8, "TAY", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xAA, "TAX", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xBA, "TSX", AddressingMode::None,             MemoryAccess::None),

    // ADC
    Instruction(0x61, "ADC", AddressingMode::IndexedIndirectX, MemoryAccess::Read),
    Instruction(0x65, "ADC", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0x6D, "ADC", AddressingMode::Absolute,         MemoryAccess::Read),
    Instruction(0x69, "ADC", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0x71, "ADC", AddressingMode::IndirectIndexedY, MemoryAccess::Read),
    Instruction(0x75, "ADC", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0x79, "ADC", AddressingMode::AbsoluteY,        MemoryAccess::Read),
    Instruction(0x7D, "ADC", AddressingMode::AbsoluteX,        MemoryAccess::Read),

    // AND
    Instruction(0x21, "AND", AddressingMode::IndexedIndirectX, MemoryAccess::Read),
    Instruction(0x25, "AND", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0x29, "AND", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0x2D, "AND", AddressingMode::Absolute,         MemoryAccess::Read),
    Instruction(0x31, "AND", AddressingMode::IndirectIndexedY, MemoryAccess::Read),
    Instruction(0x35, "AND", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0x39, "AND", AddressingMode::AbsoluteY,        MemoryAccess::Read),
    Instruction(0x3D, "AND", AddressingMode::AbsoluteX,        MemoryAccess::Read),

    // ASL
    Instruction(0x06, "ASL", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0x16, "ASL", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0x0A, "ASL", AddressingMode::Accumulator,      MemoryAccess::None),
    Instruction(0x0E, "ASL", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0x1E, "ASL", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // BIT
    Instruction(0x24, "BIT", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0x2C, "BIT", AddressingMode::Absolute,         MemoryAccess::Read),

    // CMP
    Instruction(0xC1, "CMP", AddressingMode::IndexedIndirectX, MemoryAccess::Read),
    Instruction(0xC5, "CMP", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0xC9, "CMP", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0xCD, "CMP", AddressingMode::Absolute,         MemoryAccess::Read),
    Instruction(0xD1, "CMP", AddressingMode::IndirectIndexedY, MemoryAccess::Read),
    Instruction(0xD5, "CMP", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0xD9, "CMP", AddressingMode::AbsoluteY,        MemoryAccess::Read),
    Instruction(0xDD, "CMP", AddressingMode::AbsoluteX,        MemoryAccess::Read),

    // CPX
    Instruction(0xE0, "CPX", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0xE4, "CPX", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0xEC, "CPX", AddressingMode::Absolute,         MemoryAccess::Read),

    // CPY
    Instruction(0xC0, "CPY", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0xC4, "CPY", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0xCC, "CPY", AddressingMode::Absolute,         MemoryAccess::Read),
    
    // DCP (undocumented)
    Instruction(0xC3, "DCP", AddressingMode::IndexedIndirectX, MemoryAccess::ReadWrite),
    Instruction(0xC7, "DCP", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0xCF, "DCP", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0xD3, "DCP", AddressingMode::IndirectIndexedY, MemoryAccess::ReadWrite),
    Instruction(0xD7, "DCP", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0xDB, "DCP", AddressingMode::AbsoluteY,        MemoryAccess::ReadWrite),
    Instruction(0xDF, "DCP", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // DEC
    Instruction(0xC6, "DEC", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0xCE, "DEC", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0xD6, "DEC", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0xDE, "DEC", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // EOR
    Instruction(0x41, "EOR", AddressingMode::IndexedIndirectX, MemoryAccess::Read),
    Instruction(0x45, "EOR", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0x49, "EOR", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0x4D, "EOR", AddressingMode::Absolute,         MemoryAccess::Read),
    Instruction(0x51, "EOR", AddressingMode::IndirectIndexedY, MemoryAccess::Read),
    Instruction(0x55, "EOR", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0x59, "EOR", AddressingMode::AbsoluteY,        MemoryAccess::Read),
    Instruction(0x5D, "EOR", AddressingMode::AbsoluteX,        MemoryAccess::Read),

    // INC
    Instruction(0xE6, "INC", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0xEE, "INC", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0xF6, "INC", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0xFE, "INC", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // ISB (undocumented, aka ISC)
    Instruction(0xE3, "ISB", AddressingMode::IndexedIndirectX, MemoryAccess::ReadWrite),
    Instruction(0xE7, "ISB", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0xEF, "ISB", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0xF3, "ISB", AddressingMode::IndirectIndexedY, MemoryAccess::ReadWrite),
    Instruction(0xF7, "ISB", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0xFB, "ISB", AddressingMode::AbsoluteY,        MemoryAccess::ReadWrite),
    Instruction(0xFF, "ISB", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // JAM (undocumented, aka KIL)
    Instruction(0x02, "JAM", AddressingMode::Invalid,          MemoryAccess::None),
    Instruction(0x12, "JAM", AddressingMode::Invalid,          MemoryAccess::None),
    Instruction(0x22, "JAM", AddressingMode::Invalid,          MemoryAccess::None),
    Instruction(0x32, "JAM", AddressingMode::Invalid,          MemoryAccess::None),
    Instruction(0x42, "JAM", AddressingMode::Invalid,          MemoryAccess::None),
    Instruction(0x52, "JAM", AddressingMode::Invalid,          MemoryAccess::None),
    Instruction(0x62, "JAM", AddressingMode::Invalid,          MemoryAccess::None),
    Instruction(0x72, "JAM", AddressingMode::Invalid,          MemoryAccess::None),
    Instruction(0x92, "JAM", AddressingMode::Invalid,          MemoryAccess::None),
    Instruction(0xB2, "JAM", AddressingMode::Invalid,          MemoryAccess::None),
    Instruction(0xD2, "JAM", AddressingMode::Invalid,          MemoryAccess::None),
    Instruction(0xF2, "JAM", AddressingMode::Invalid,          MemoryAccess::None),

    // LAX (undocumented, LDA + LAX)
    Instruction(0xA3, "LAX", AddressingMode::IndexedIndirectX, MemoryAccess::Read),
    Instruction(0xA7, "LAX", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0xAF, "LAX", AddressingMode::Absolute,         MemoryAccess::Read),
    Instruction(0xB3, "LAX", AddressingMode::IndirectIndexedY, MemoryAccess::Read),
    Instruction(0xB7, "LAX", AddressingMode::ZeroPageY,        MemoryAccess::Read),
    Instruction(0xBF, "LAX", AddressingMode::AbsoluteY,        MemoryAccess::Read),

    // LDA
    Instruction(0xA1, "LDA", AddressingMode::IndexedIndirectX, MemoryAccess::Read),
    Instruction(0xA5, "LDA", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0xA9, "LDA", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0xAD, "LDA", AddressingMode::Absolute,         MemoryAccess::Read),
    Instruction(0xB1, "LDA", AddressingMode::IndirectIndexedY, MemoryAccess::Read),
    Instruction(0xB5, "LDA", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0xB9, "LDA", AddressingMode::AbsoluteY,        MemoryAccess::Read),
    Instruction(0xBD, "LDA", AddressingMode::AbsoluteX,        MemoryAccess::Read),

    // LDX
    Instruction(0xA2, "LDX", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0xA6, "LDX", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0xAE, "LDX", AddressingMode::Absolute,         MemoryAccess::Read),
    Instruction(0xB6, "LDX", AddressingMode::ZeroPageY,        MemoryAccess::Read),
    Instruction(0xBE, "LDX", AddressingMode::AbsoluteY,        MemoryAccess::Read),

    // LDY
    Instruction(0xA0, "LDY", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0xA4, "LDY", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0xAC, "LDY", AddressingMode::Absolute,         MemoryAccess::Read),
    Instruction(0xB4, "LDY", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0xBC, "LDY", AddressingMode::AbsoluteX,        MemoryAccess::Read),

    // LSR
    Instruction(0x46, "LSR", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0x4A, "LSR", AddressingMode::Accumulator,      MemoryAccess::None),
    Instruction(0x4E, "LSR", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0x56, "LSR", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0x5E, "LSR", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // NOP
    Instruction(0x04, "NOP", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0x0C, "NOP", AddressingMode::Absolute,         MemoryAccess::Read),
    Instruction(0x14, "NOP", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0x1A, "NOP", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x1C, "NOP", AddressingMode::AbsoluteX,        MemoryAccess::Read),
    Instruction(0x34, "NOP", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0x3A, "NOP", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x3C, "NOP", AddressingMode::AbsoluteX,        MemoryAccess::Read),
    Instruction(0x44, "NOP", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0x54, "NOP", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0x5A, "NOP", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x5C, "NOP", AddressingMode::AbsoluteX,        MemoryAccess::Read),
    Instruction(0x64, "NOP", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0x74, "NOP", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0x7A, "NOP", AddressingMode::None,             MemoryAccess::None),
    Instruction(0x7C, "NOP", AddressingMode::AbsoluteX,        MemoryAccess::Read),
    Instruction(0x80, "NOP", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0xD4, "NOP", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0xDA, "NOP", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xDC, "NOP", AddressingMode::AbsoluteX,        MemoryAccess::Read),
    Instruction(0xEA, "NOP", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xF4, "NOP", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0xFA, "NOP", AddressingMode::None,             MemoryAccess::None),
    Instruction(0xFC, "NOP", AddressingMode::AbsoluteX,        MemoryAccess::Read),
    
    // ORA
    Instruction(0x01, "ORA", AddressingMode::IndexedIndirectX, MemoryAccess::Read),
    Instruction(0x05, "ORA", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0x09, "ORA", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0x0D, "ORA", AddressingMode::Absolute,         MemoryAccess::Read),
    Instruction(0x11, "ORA", AddressingMode::IndirectIndexedY, MemoryAccess::Read),
    Instruction(0x15, "ORA", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0x19, "ORA", AddressingMode::AbsoluteY,        MemoryAccess::Read),
    Instruction(0x1D, "ORA", AddressingMode::AbsoluteX,        MemoryAccess::Read),

    // RLA (undocumented, ROL + AND)
    Instruction(0x23, "RLA", AddressingMode::IndexedIndirectX, MemoryAccess::ReadWrite),
    Instruction(0x27, "RLA", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0x2F, "RLA", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0x33, "RLA", AddressingMode::IndirectIndexedY, MemoryAccess::ReadWrite),
    Instruction(0x37, "RLA", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0x3B, "RLA", AddressingMode::AbsoluteY,        MemoryAccess::ReadWrite),
    Instruction(0x3F, "RLA", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // ROL
    Instruction(0x26, "ROL", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0x2A, "ROL", AddressingMode::Accumulator,      MemoryAccess::None),
    Instruction(0x2E, "ROL", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0x36, "ROL", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0x3E, "ROL", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // ROR
    Instruction(0x66, "ROR", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0x6A, "ROR", AddressingMode::Accumulator,      MemoryAccess::None),
    Instruction(0x6E, "ROR", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0x76, "ROR", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0x7E, "ROR", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // RRA (undocumented, ROR + ADC)
    Instruction(0x63, "RRA", AddressingMode::IndexedIndirectX, MemoryAccess::ReadWrite),
    Instruction(0x67, "RRA", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0x6F, "RRA", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0x73, "RRA", AddressingMode::IndirectIndexedY, MemoryAccess::ReadWrite),
    Instruction(0x77, "RRA", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0x7B, "RRA", AddressingMode::AbsoluteY,        MemoryAccess::ReadWrite),
    Instruction(0x7F, "RRA", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // SAX (undocumented)
    Instruction(0x83, "SAX", AddressingMode::IndexedIndirectX, MemoryAccess::Write),
    Instruction(0x87, "SAX", AddressingMode::ZeroPage,         MemoryAccess::Write),
    Instruction(0x8F, "SAX", AddressingMode::Absolute,         MemoryAccess::Write),
    Instruction(0x97, "SAX", AddressingMode::ZeroPageY,        MemoryAccess::Write),

    // SBC
    Instruction(0xE1, "SBC", AddressingMode::IndexedIndirectX, MemoryAccess::Read),
    Instruction(0xE5, "SBC", AddressingMode::ZeroPage,         MemoryAccess::Read),
    Instruction(0xE9, "SBC", AddressingMode::Immediate,        MemoryAccess::None),
    Instruction(0xEB, "SBC", AddressingMode::Immediate,        MemoryAccess::None), // Undocumented
    Instruction(0xED, "SBC", AddressingMode::Absolute,         MemoryAccess::Read),
    Instruction(0xF1, "SBC", AddressingMode::IndirectIndexedY, MemoryAccess::Read),
    Instruction(0xF5, "SBC", AddressingMode::ZeroPageX,        MemoryAccess::Read),
    Instruction(0xF9, "SBC", AddressingMode::AbsoluteY,        MemoryAccess::Read),
    Instruction(0xFD, "SBC", AddressingMode::AbsoluteX,        MemoryAccess::Read),

    // SLO (undocumented, ASL + ORA)
    Instruction(0x03, "SLO", AddressingMode::IndexedIndirectX, MemoryAccess::ReadWrite),
    Instruction(0x07, "SLO", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0x0F, "SLO", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0x13, "SLO", AddressingMode::IndirectIndexedY, MemoryAccess::ReadWrite),
    Instruction(0x17, "SLO", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0x1B, "SLO", AddressingMode::AbsoluteY,        MemoryAccess::ReadWrite),
    Instruction(0x1F, "SLO", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // SRE (undocumented, LSR + EOR)
    Instruction(0x43, "SRE", AddressingMode::IndexedIndirectX, MemoryAccess::ReadWrite),
    Instruction(0x47, "SRE", AddressingMode::ZeroPage,         MemoryAccess::ReadWrite),
    Instruction(0x4F, "SRE", AddressingMode::Absolute,         MemoryAccess::ReadWrite),
    Instruction(0x53, "SRE", AddressingMode::IndirectIndexedY, MemoryAccess::ReadWrite),
    Instruction(0x57, "SRE", AddressingMode::ZeroPageX,        MemoryAccess::ReadWrite),
    Instruction(0x5B, "SRE", AddressingMode::AbsoluteY,        MemoryAccess::ReadWrite),
    Instruction(0x5F, "SRE", AddressingMode::AbsoluteX,        MemoryAccess::ReadWrite),

    // STA
    Instruction(0x81, "STA", AddressingMode::IndexedIndirectX, MemoryAccess::Write),
    Instruction(0x85, "STA", AddressingMode::ZeroPage,         MemoryAccess::Write),
    Instruction(0x8D, "STA", AddressingMode::Absolute,         MemoryAccess::Write),
    Instruction(0x91, "STA", AddressingMode::IndirectIndexedY, MemoryAccess::Write),
    Instruction(0x95, "STA", AddressingMode::ZeroPageX,        MemoryAccess::Write),
    Instruction(0x99, "STA", AddressingMode::AbsoluteY,        MemoryAccess::Write),
    Instruction(0x9D, "STA", AddressingMode::AbsoluteX,        MemoryAccess::Write),

    // STX
    Instruction(0x86, "STX", AddressingMode::ZeroPage,         MemoryAccess::Write),
    Instruction(0x8E, "STX", AddressingMode::Absolute,         MemoryAccess::Write),
    Instruction(0x96, "STX", AddressingMode::ZeroPageY,        MemoryAccess::Write),

    // STY
    Instruction(0x84, "STY", AddressingMode::ZeroPage,         MemoryAccess::Write),
    Instruction(0x8C, "STY", AddressingMode::Absolute,         MemoryAccess::Write),
    Instruction(0x94, "STY", AddressingMode::ZeroPageX,        MemoryAccess::Write),    
];

struct InstructionCodeBuilder {
    lines: Vec<String>,
}

impl InstructionCodeBuilder {
    fn new() -> InstructionCodeBuilder {
        InstructionCodeBuilder {
            lines: Vec::with_capacity(8),
        }
    }

    fn add(&mut self, text: &'static str) {
        self.lines.push(text.to_string());
    }

    fn add_string(&mut self, text: String) {
        self.lines.push(text);
    }

    fn modify_previous(&mut self, text: &str) {
        let num_lines = self.lines.len();
        let line = &mut self.lines[num_lines - 1];
        if line != "" {
            *line += " ";
        }
        *line += text;
    }

    fn modify_previous_string(&mut self, text: String) {
        self.modify_previous(text.as_str());
    }

    fn encode_operation(&mut self, mnemonic: &str, addressing_mode: &AddressingMode) {
        match mnemonic {
            // Special cases
            "BRK" => {
                self.add("brk_0(&mut self.registers, &mut pins);");
                self.add("brk_1(&mut self.registers, &mut pins);");
                self.add("brk_2(&mut self.registers, &mut pins);");
                self.add("brk_3(&mut self.registers, &mut pins);");
                self.add("brk_4(&mut self.registers, &mut pins);");
                self.add("brk_5(&mut self.registers, &mut pins);");
            },
            "JMP" => self.modify_previous("jmp(&mut self.registers, &pins);"),
            "JSR" => {
                self.add("jsr_0(&mut self.registers, &mut pins);");
                self.add("jsr_1(&mut self.registers, &mut pins);");
                self.add("jsr_2(&mut self.registers, &mut pins);");
                self.add("jsr_3(&mut self.registers, &mut pins);");
                self.add("jsr_4(&mut self.registers, &mut pins);");
                self.add("jsr_5(&mut self.registers, &mut pins);");
            },
            "PLA" => {
                self.add("pla_0(&mut self.registers, &mut pins);");
                self.add("pla_1(&mut self.registers, &mut pins);");
                self.add("pla_2(&mut self.registers, &mut pins);");
            },
            "PLP" => {
                self.add("plp_0(&mut self.registers, &mut pins);");
                self.add("plp_1(&mut self.registers, &mut pins);");
                self.add("plp_2(&mut self.registers, &mut pins);");
            },
            "RTI" => {
                self.add("rti_0(&mut self.registers, &mut pins);");
                self.add("rti_1(&mut self.registers, &mut pins);");
                self.add("rti_2(&mut self.registers, &mut pins);");
                self.add("rti_3(&mut self.registers, &mut pins);");
                self.add("rti_4(&mut self.registers, &mut pins);");
            },
            "RTS" => {
                self.add("rts_0(&mut self.registers, &mut pins);");
                self.add("rts_1(&mut self.registers, &mut pins);");
                self.add("rts_2(&mut self.registers, &mut pins);");
                self.add("rts_3(&mut self.registers, &mut pins);");
                self.add("");
            },
            "CLC" | "CLD" | "CLI" | "CLV" | "SED" | "SEI" | "SLC" |
            "DEX" | "DEY" | "INX" | "INY" | 
            "TAX" | "TAY" | "TSX" | "TXA" | "TXS" | "TYA" => {
                self.add_string(format!("{}(&mut self.registers);", mnemonic.to_lowercase()))
            },

            // Write memory
            "PHA" | "PHP" | "SAX" | "STA" | "STX" | "STY" => {
                self.modify_previous_string(format!("{}(&mut self.registers, &mut pins);", mnemonic.to_lowercase()));
            },

            // Read memory
            "ADC" | "AND" | "BIT" | "CMP" | "CPX" | "CPY" | "EOR" | "LAX" | "LDA" | "LDX" | "LDY" | "ORA" | "SBC" => {
                self.add_string(format!("{}(&mut self.registers, &pins);", mnemonic.to_lowercase()))
            },

            // Accumulator
            "ASL" | "LSR" | "ROL" | "ROR" if addressing_mode == &AddressingMode::Accumulator => {
                self.add_string(format!("{}a(&mut self.registers);", mnemonic.to_lowercase()));
            },

            // Read / modify / write memory
            "ASL" | "DEC" | "DCP" | "INC" | "ISB" | "LSR" | "RLA" | "ROL" | "ROR" | "RRA" | "SLO" | "SRE" => {
                self.add_string(format!("{}_0(&mut self.registers, &mut pins);", mnemonic.to_lowercase()));
                self.add_string(format!("{}_1(&mut self.registers, &mut pins);", mnemonic.to_lowercase()));
            },

            // Branch
            "BCC" | "BCS" | "BEQ" | "BMI" | "BNE" | "BPL" | "BVC" | "BVS" => {
                self.add_string(format!("branch_0_{}(&mut self.registers, &mut pins);", mnemonic.to_lowercase()));
                self.add("branch_1(&mut self.registers, &mut pins);");
                self.add("branch_2(&mut self.registers);");
            },

            // Invalid
            "JAM" => {},

            "NOP" => self.add(""),
            
            _ => unreachable!("Unexpected mnemonic {}", mnemonic)
        }
    }

    fn encode_addressing_mode(&mut self, instruction: &Instruction) {
        match instruction.2 {
            AddressingMode::None | AddressingMode::Accumulator => {
                self.add("addressing_mode_none_cycle_0(&mut self.registers, &mut pins);");
            },
            AddressingMode::Immediate => {
                self.add("addressing_mode_immediate_cycle_0(&mut self.registers, &mut pins);");
            },
            AddressingMode::ZeroPage => {
                self.add("addressing_mode_zero_page_cycle_0(&mut self.registers, &mut pins);");
                self.add("addressing_mode_zero_page_cycle_1(&mut pins);");
            },
            AddressingMode::ZeroPageX => {
                self.add("addressing_mode_zero_page_indexed_cycle_0(&mut self.registers, &mut pins);");
                self.add("addressing_mode_zero_page_indexed_cycle_1(&mut self.registers, &mut pins);");
                self.add("addressing_mode_zero_page_x_cycle_2(&mut self.registers, &mut pins);");
            },
            AddressingMode::ZeroPageY => {
                self.add("addressing_mode_zero_page_indexed_cycle_0(&mut self.registers, &mut pins);");
                self.add("addressing_mode_zero_page_indexed_cycle_1(&mut self.registers, &mut pins);");
                self.add("addressing_mode_zero_page_y_cycle_2(&mut self.registers, &mut pins);");
            },
            AddressingMode::Absolute => {
                self.add("addressing_mode_absolute_cycle_0(&mut self.registers, &mut pins);");
                self.add("addressing_mode_absolute_cycle_1(&mut self.registers, &mut pins);");
                self.add("addressing_mode_absolute_cycle_2(&mut self.registers, &mut pins);");
            },
            AddressingMode::AbsoluteX => {
                self.add("addressing_mode_absolute_indexed_cycle_0(&mut self.registers, &mut pins);");
                self.add("addressing_mode_absolute_indexed_cycle_1(&mut self.registers, &mut pins);");
                self.add("addressing_mode_absolute_indexed_cycle_2(self.registers.x, &mut self.registers, &mut pins);");
                if instruction.3 == MemoryAccess::Read {
                    self.modify_previous("addressing_mode_absolute_indexed_cycle_2_read(self.registers.x, &mut self.registers);");
                }
                self.add("addressing_mode_absolute_indexed_cycle_3(self.registers.x, &mut self.registers, &mut pins);");
            },
            AddressingMode::AbsoluteY => {
                self.add("addressing_mode_absolute_indexed_cycle_0(&mut self.registers, &mut pins);");
                self.add("addressing_mode_absolute_indexed_cycle_1(&mut self.registers, &mut pins);");
                self.add("addressing_mode_absolute_indexed_cycle_2(self.registers.y, &mut self.registers, &mut pins);");
                if instruction.3 == MemoryAccess::Read {
                    self.modify_previous("addressing_mode_absolute_indexed_cycle_2_read(self.registers.y, &mut self.registers);");
                }
                self.add("addressing_mode_absolute_indexed_cycle_3(self.registers.y, &mut self.registers, &mut pins);");
            },
            AddressingMode::IndexedIndirectX => {
                self.add("addressing_mode_indexed_indirect_x_cycle_0(&mut self.registers, &mut pins);");
                self.add("addressing_mode_indexed_indirect_x_cycle_1(&mut self.registers, &mut pins);");
                self.add("addressing_mode_indexed_indirect_x_cycle_2(&mut self.registers, &mut pins);");
                self.add("addressing_mode_indexed_indirect_x_cycle_3(&mut self.registers, &mut pins);");
                self.add("addressing_mode_indexed_indirect_x_cycle_4(&mut self.registers, &mut pins);");
            },
            AddressingMode::IndirectIndexedY => {
                self.add("addressing_mode_indirect_indexed_y_cycle_0(&mut self.registers, &mut pins);");
                self.add("addressing_mode_indirect_indexed_y_cycle_1(&mut self.registers, &mut pins);");
                self.add("addressing_mode_indirect_indexed_y_cycle_2(&mut self.registers, &mut pins);");
                self.add("addressing_mode_indirect_indexed_y_cycle_3(&mut self.registers, &mut pins);");
                if instruction.3 == MemoryAccess::Read {
                    self.modify_previous("addressing_mode_indirect_indexed_y_cycle_3_read(&mut self.registers);");
                }
                self.add("addressing_mode_indirect_indexed_y_cycle_4(&mut self.registers, &mut pins);");
            },
            AddressingMode::Indirect => {
                self.add("addressing_mode_indirect_cycle_0(&mut self.registers, &mut pins);");
                self.add("addressing_mode_indirect_cycle_1(&mut self.registers, &mut pins);");
                self.add("addressing_mode_indirect_cycle_2(&mut self.registers, &mut pins);");
                self.add("addressing_mode_indirect_cycle_3(&mut self.registers, &mut pins);");
                self.add("addressing_mode_indirect_cycle_4(&mut self.registers, &mut pins);");
            },
            AddressingMode::JSR => (),
            AddressingMode::Invalid => {
                self.add("addressing_mode_invalid_cycle_0(&mut self.registers, &mut pins);");
                self.add("addressing_mode_invalid_cycle_1(&mut self.registers, &mut pins);");
            },
        }
    }
}

struct InstructionCode {
    comment: String,
    lines: Vec<String>,
}

impl InstructionCode {
    fn from_instruction(instruction: &Instruction) -> InstructionCode {
        let comment = format!("{0} {1}", instruction.1, instruction.2.as_string());
    
        let mut code_builder = InstructionCodeBuilder::new();
        code_builder.encode_addressing_mode(instruction);

        code_builder.encode_operation(instruction.1, &instruction.2);
    
        match instruction.3 {
            MemoryAccess::None | MemoryAccess::Read => code_builder.modify_previous("pins.fetch_next_instruction(&self.registers);"),
            _ => code_builder.add("pins.fetch_next_instruction(&self.registers);"),
        }
    
        InstructionCode {
            comment,
            lines: code_builder.lines,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("mos6502_instructions.generated.rs");
    let mut buffer = File::create(&dest_path)?;

    write!(buffer, "// This is a generated file. Do not modify.\n")?;
    write!(buffer, "\n")?;
    write!(buffer, "match (self.registers.ir, self.registers.tr) {{\n")?;

    for instruction in INSTRUCTIONS.iter() {
        let instruction_code = InstructionCode::from_instruction(instruction);

        write!(buffer, "    // {}\n", instruction_code.comment)?;

        for (index, line) in instruction_code.lines.iter().enumerate() {
            if line == "" { break; }

            write!(buffer, "    (0x{:02X}, {}) => {{ {} }},\n", instruction.0, index, line)?;
        }

        write!(buffer, "\n")?;
    }

    write!(buffer, "    _ => todo!(\"Unimplemented opcode 0x{{:02X}} timing {{}}\", self.registers.ir, self.registers.tr)")?;

    write!(buffer, "}}\n")?;

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}