use crate::Registers;

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
    sp: u16,
    interrupt_enable_flag: bool,
    halt: bool,
    ei: u32,
    di: u32
}

struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&self, addr: u16, byte: u8) {
        self.memory[addr as usize]=byte
    }

    fn read_word(&self, address: u16) -> u16 {
        u16::from(self.read_byte(address)) | (u16::from(self.read_byte(address + 1)) << 8)
    }

    fn write_word(&mut self, addr: u16, word: u16) {
        self.memory[addr as usize] = (word & 0xFF) as u8;
        self.memory[(addr + 1) as usize] = ((word >> 8) & 0xFF) as u8;
    }
}

enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

enum JumpCondition{
    Address16, AddressHL
}

enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource)
}

enum LoadByteTarget{
    A, B, C, D, E, H, L, AddressHL, AddressBC, AddressDE, AddressHLP, AddressHLM, AddressC, Address16, Address8
}

enum LoadByteSource{
    A, B, C, D, E, H, L, AddressHL, AddressBC, AddressDE, AddressHLP, AddressHLM, AddressC, D8, Address8
}

enum LoadWordTarget{
    BC, DE, HL, SP, Address16
}

enum LoadWordSource{
    D16, SP, HL, SPR8
}

enum StackTarget {AF, HL , BC, DE}


enum RstTarget{
    Rst00H, Rst08H, Rst10H, Rst18H, Rst20H, Rst28H, Rst30H, Rst38H,
}

enum Instruction {
    NOP,
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADDSP(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    CCF,
    SCF,
    DAA,
    CPL,
    RRA,
    RLA,
    RRCA,
    RLCA,
    BIT(ArithmeticTarget, u8), // ArithmeticTarget represents the register to test and u8 represents the bit number (0-7).
    RESET(ArithmeticTarget, u8), // ArithmeticTarget represents the register to reset and u8 represents the bit number (0-7).
    SET(ArithmeticTarget, u8), // ArithmeticTarget represents the register to set and u8 represents the bit number (0-7).
    SRL(ArithmeticTarget),
    RR(ArithmeticTarget),
    RL(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RLC(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
    LD(LoadType),
    LDH(LoadType),
    JP(JumpTest, JumpCondition),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest, JumpCondition),
    RET(JumpTest),
    JPI,
    JR(JumpTest),
    RETI,
    STOP,
    HALT,
    EI,
    DI,
    RST(RstTarget)
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L, HL, AF, BC, DE, SP, AddressHL, D8
}


impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(ArithmeticTarget::B)),
            0x01 => Some(Instruction::RLC(ArithmeticTarget::C)),
            0x02 => Some(Instruction::RLC(ArithmeticTarget::D)),
            0x03 => Some(Instruction::RLC(ArithmeticTarget::E)),
            0x04 => Some(Instruction::RLC(ArithmeticTarget::H)),
            0x05 => Some(Instruction::RLC(ArithmeticTarget::L)),
            0x06 => Some(Instruction::RLC(ArithmeticTarget::AddressHL)),
            0x07 => Some(Instruction::RLC(ArithmeticTarget::A)),
            0x08 => Some(Instruction::RRC(ArithmeticTarget::B)),
            0x09 => Some(Instruction::RRC(ArithmeticTarget::C)),
            0x0A => Some(Instruction::RRC(ArithmeticTarget::D)),
            0x0B => Some(Instruction::RRC(ArithmeticTarget::E)),
            0x0C => Some(Instruction::RRC(ArithmeticTarget::H)),
            0x0D => Some(Instruction::RRC(ArithmeticTarget::L)),
            0x0E => Some(Instruction::RRC(ArithmeticTarget::AddressHL)),
            0x0F => Some(Instruction::RRC(ArithmeticTarget::A)),

            0x10 => Some(Instruction::RL(ArithmeticTarget::B)),
            0x11 => Some(Instruction::RL(ArithmeticTarget::C)),
            0x12 => Some(Instruction::RL(ArithmeticTarget::D)),
            0x13 => Some(Instruction::RL(ArithmeticTarget::E)),
            0x14 => Some(Instruction::RL(ArithmeticTarget::H)),
            0x15 => Some(Instruction::RL(ArithmeticTarget::L)),
            0x16 => Some(Instruction::RL(ArithmeticTarget::AddressHL)),
            0x17 => Some(Instruction::RL(ArithmeticTarget::A)),
            0x18 => Some(Instruction::RR(ArithmeticTarget::B)),
            0x19 => Some(Instruction::RR(ArithmeticTarget::C)),
            0x1A => Some(Instruction::RR(ArithmeticTarget::D)),
            0x1B => Some(Instruction::RR(ArithmeticTarget::E)),
            0x1C => Some(Instruction::RR(ArithmeticTarget::H)),
            0x1D => Some(Instruction::RR(ArithmeticTarget::L)),
            0x1E => Some(Instruction::RR(ArithmeticTarget::AddressHL)),
            0x1F => Some(Instruction::RR(ArithmeticTarget::A)),

            0x20 => Some(Instruction::SLA(ArithmeticTarget::B)),
            0x21 => Some(Instruction::SLA(ArithmeticTarget::C)),
            0x22 => Some(Instruction::SLA(ArithmeticTarget::D)),
            0x23 => Some(Instruction::SLA(ArithmeticTarget::E)),
            0x24 => Some(Instruction::SLA(ArithmeticTarget::H)),
            0x25 => Some(Instruction::SLA(ArithmeticTarget::L)),
            0x26 => Some(Instruction::SLA(ArithmeticTarget::AddressHL)),
            0x27 => Some(Instruction::SLA(ArithmeticTarget::A)),
            0x28 => Some(Instruction::SRA(ArithmeticTarget::B)),
            0x29 => Some(Instruction::SRA(ArithmeticTarget::C)),
            0x2A => Some(Instruction::SRA(ArithmeticTarget::D)),
            0x2B => Some(Instruction::SRA(ArithmeticTarget::E)),
            0x2C => Some(Instruction::SRA(ArithmeticTarget::H)),
            0x2D => Some(Instruction::SRA(ArithmeticTarget::L)),
            0x2E => Some(Instruction::SRA(ArithmeticTarget::AddressHL)),
            0x2F => Some(Instruction::SRA(ArithmeticTarget::A)),

            0x30 => Some(Instruction::SWAP(ArithmeticTarget::B)),
            0x31 => Some(Instruction::SWAP(ArithmeticTarget::C)),
            0x32 => Some(Instruction::SWAP(ArithmeticTarget::D)),
            0x33 => Some(Instruction::SWAP(ArithmeticTarget::E)),
            0x34 => Some(Instruction::SWAP(ArithmeticTarget::H)),
            0x35 => Some(Instruction::SWAP(ArithmeticTarget::L)),
            0x36 => Some(Instruction::SWAP(ArithmeticTarget::AddressHL)),
            0x37 => Some(Instruction::SWAP(ArithmeticTarget::A)),
            0x38 => Some(Instruction::SRL(ArithmeticTarget::B)),
            0x39 => Some(Instruction::SRL(ArithmeticTarget::C)),
            0x3A => Some(Instruction::SRL(ArithmeticTarget::D)),
            0x3B => Some(Instruction::SRL(ArithmeticTarget::E)),
            0x3C => Some(Instruction::SRL(ArithmeticTarget::H)),
            0x3D => Some(Instruction::SRL(ArithmeticTarget::L)),
            0x3E => Some(Instruction::SRL(ArithmeticTarget::AddressHL)),
            0x3F => Some(Instruction::SRL(ArithmeticTarget::A)),

            0x40 => Some(Instruction::BIT(ArithmeticTarget::B, 0)),
            0x41 => Some(Instruction::BIT(ArithmeticTarget::C, 0)),
            0x42 => Some(Instruction::BIT(ArithmeticTarget::D, 0)),
            0x43 => Some(Instruction::BIT(ArithmeticTarget::E, 0)),
            0x44 => Some(Instruction::BIT(ArithmeticTarget::H, 0)),
            0x45 => Some(Instruction::BIT(ArithmeticTarget::L, 0)),
            0x46 => Some(Instruction::BIT(ArithmeticTarget::AddressHL, 0)),
            0x47 => Some(Instruction::BIT(ArithmeticTarget::A, 0)),
            0x48 => Some(Instruction::BIT(ArithmeticTarget::B, 1)),
            0x49 => Some(Instruction::BIT(ArithmeticTarget::C, 1)),
            0x4A => Some(Instruction::BIT(ArithmeticTarget::D, 1)),
            0x4B => Some(Instruction::BIT(ArithmeticTarget::E, 1)),
            0x4C => Some(Instruction::BIT(ArithmeticTarget::H, 1)),
            0x4D => Some(Instruction::BIT(ArithmeticTarget::L, 1)),
            0x4E => Some(Instruction::BIT(ArithmeticTarget::AddressHL, 1)),
            0x4F => Some(Instruction::BIT(ArithmeticTarget::A, 1)),

            // BIT instruction (bit 2)
            0x50 => Some(Instruction::BIT(ArithmeticTarget::B, 2)),
            0x51 => Some(Instruction::BIT(ArithmeticTarget::C, 2)),
            0x52 => Some(Instruction::BIT(ArithmeticTarget::D, 2)),
            0x53 => Some(Instruction::BIT(ArithmeticTarget::E, 2)),
            0x54 => Some(Instruction::BIT(ArithmeticTarget::H, 2)),
            0x55 => Some(Instruction::BIT(ArithmeticTarget::L, 2)),
            0x56 => Some(Instruction::BIT(ArithmeticTarget::AddressHL, 2)),
            0x57 => Some(Instruction::BIT(ArithmeticTarget::A, 2)),

            // BIT instruction (bit 3)
            0x58 => Some(Instruction::BIT(ArithmeticTarget::B, 3)),
            0x59 => Some(Instruction::BIT(ArithmeticTarget::C, 3)),
            0x5A => Some(Instruction::BIT(ArithmeticTarget::D, 3)),
            0x5B => Some(Instruction::BIT(ArithmeticTarget::E, 3)),
            0x5C => Some(Instruction::BIT(ArithmeticTarget::H, 3)),
            0x5D => Some(Instruction::BIT(ArithmeticTarget::L, 3)),
            0x5E => Some(Instruction::BIT(ArithmeticTarget::AddressHL, 3)),
            0x5F => Some(Instruction::BIT(ArithmeticTarget::A, 3)),

            // BIT instruction (bit 4)
            0x60 => Some(Instruction::BIT(ArithmeticTarget::B, 4)),
            0x61 => Some(Instruction::BIT(ArithmeticTarget::C, 4)),
            0x62 => Some(Instruction::BIT(ArithmeticTarget::D, 4)),
            0x63 => Some(Instruction::BIT(ArithmeticTarget::E, 4)),
            0x64 => Some(Instruction::BIT(ArithmeticTarget::H, 4)),
            0x65 => Some(Instruction::BIT(ArithmeticTarget::L, 4)),
            0x66 => Some(Instruction::BIT(ArithmeticTarget::AddressHL, 4)),
            0x67 => Some(Instruction::BIT(ArithmeticTarget::A, 4)),

            // BIT instruction (bit 5)
            0x68 => Some(Instruction::BIT(ArithmeticTarget::B, 5)),
            0x69 => Some(Instruction::BIT(ArithmeticTarget::C, 5)),
            0x6A => Some(Instruction::BIT(ArithmeticTarget::D, 5)),
            0x6B => Some(Instruction::BIT(ArithmeticTarget::E, 5)),
            0x6C => Some(Instruction::BIT(ArithmeticTarget::H, 5)),
            0x6D => Some(Instruction::BIT(ArithmeticTarget::L, 5)),
            0x6E => Some(Instruction::BIT(ArithmeticTarget::AddressHL, 5)),
            0x6F => Some(Instruction::BIT(ArithmeticTarget::A, 5)),

            // BIT instruction (bit 6)
            0x70 => Some(Instruction::BIT(ArithmeticTarget::B, 6)),
            0x71 => Some(Instruction::BIT(ArithmeticTarget::C, 6)),
            0x72 => Some(Instruction::BIT(ArithmeticTarget::D, 6)),
            0x73 => Some(Instruction::BIT(ArithmeticTarget::E, 6)),
            0x74 => Some(Instruction::BIT(ArithmeticTarget::H, 6)),
            0x75 => Some(Instruction::BIT(ArithmeticTarget::L, 6)),
            0x76 => Some(Instruction::BIT(ArithmeticTarget::AddressHL, 6)),
            0x77 => Some(Instruction::BIT(ArithmeticTarget::A, 6)),

            // BIT instruction (bit 7)
            0x78 => Some(Instruction::BIT(ArithmeticTarget::B, 7)),
            0x79 => Some(Instruction::BIT(ArithmeticTarget::C, 7)),
            0x7A => Some(Instruction::BIT(ArithmeticTarget::D, 7)),
            0x7B => Some(Instruction::BIT(ArithmeticTarget::E, 7)),
            0x7C => Some(Instruction::BIT(ArithmeticTarget::H, 7)),
            0x7D => Some(Instruction::BIT(ArithmeticTarget::L, 7)),
            0x7E => Some(Instruction::BIT(ArithmeticTarget::AddressHL, 7)),
            0x7F => Some(Instruction::BIT(ArithmeticTarget::A, 7)),

            // RESET instruction (bit 0)
            0x80 => Some(Instruction::RESET(ArithmeticTarget::B, 0)),
            0x81 => Some(Instruction::RESET(ArithmeticTarget::C, 0)),
            0x82 => Some(Instruction::RESET(ArithmeticTarget::D, 0)),
            0x83 => Some(Instruction::RESET(ArithmeticTarget::E, 0)),
            0x84 => Some(Instruction::RESET(ArithmeticTarget::H, 0)),
            0x85 => Some(Instruction::RESET(ArithmeticTarget::L, 0)),
            0x86 => Some(Instruction::RESET(ArithmeticTarget::AddressHL, 0)),
            0x87 => Some(Instruction::RESET(ArithmeticTarget::A, 0)),

            // RESET instruction (bit 1)
            0x88 => Some(Instruction::RESET(ArithmeticTarget::B, 1)),
            0x89 => Some(Instruction::RESET(ArithmeticTarget::C, 1)),
            0x8A => Some(Instruction::RESET(ArithmeticTarget::D, 1)),
            0x8B => Some(Instruction::RESET(ArithmeticTarget::E, 1)),
            0x8C => Some(Instruction::RESET(ArithmeticTarget::H, 1)),
            0x8D => Some(Instruction::RESET(ArithmeticTarget::L, 1)),
            0x8E => Some(Instruction::RESET(ArithmeticTarget::AddressHL, 1)),
            0x8F => Some(Instruction::RESET(ArithmeticTarget::A, 1)),

            // RESET instruction (bit 2)
            0x90 => Some(Instruction::RESET(ArithmeticTarget::B, 2)),
            0x91 => Some(Instruction::RESET(ArithmeticTarget::C, 2)),
            0x92 => Some(Instruction::RESET(ArithmeticTarget::D, 2)),
            0x93 => Some(Instruction::RESET(ArithmeticTarget::E, 2)),
            0x94 => Some(Instruction::RESET(ArithmeticTarget::H, 2)),
            0x95 => Some(Instruction::RESET(ArithmeticTarget::L, 2)),
            0x96 => Some(Instruction::RESET(ArithmeticTarget::AddressHL, 2)),
            0x97 => Some(Instruction::RESET(ArithmeticTarget::A, 2)),

            // RESET instruction (bit 3)
            0x98 => Some(Instruction::RESET(ArithmeticTarget::B, 3)),
            0x99 => Some(Instruction::RESET(ArithmeticTarget::C, 3)),
            0x9A => Some(Instruction::RESET(ArithmeticTarget::D, 3)),
            0x9B => Some(Instruction::RESET(ArithmeticTarget::E, 3)),
            0x9C => Some(Instruction::RESET(ArithmeticTarget::H, 3)),
            0x9D => Some(Instruction::RESET(ArithmeticTarget::L, 3)),
            0x9E => Some(Instruction::RESET(ArithmeticTarget::AddressHL, 3)),
            0x9F => Some(Instruction::RESET(ArithmeticTarget::A, 3)),

            // RESET instruction (bit 4)
            0xA0 => Some(Instruction::RESET(ArithmeticTarget::B, 4)),
            0xA1 => Some(Instruction::RESET(ArithmeticTarget::C, 4)),
            0xA2 => Some(Instruction::RESET(ArithmeticTarget::D, 4)),
            0xA3 => Some(Instruction::RESET(ArithmeticTarget::E, 4)),
            0xA4 => Some(Instruction::RESET(ArithmeticTarget::H, 4)),
            0xA5 => Some(Instruction::RESET(ArithmeticTarget::L, 4)),
            0xA6 => Some(Instruction::RESET(ArithmeticTarget::AddressHL, 4)),
            0xA7 => Some(Instruction::RESET(ArithmeticTarget::A, 4)),

            // RESET instruction (bit 5)
            0xA8 => Some(Instruction::RESET(ArithmeticTarget::B, 5)),
            0xA9 => Some(Instruction::RESET(ArithmeticTarget::C, 5)),
            0xAA => Some(Instruction::RESET(ArithmeticTarget::D, 5)),
            0xAB => Some(Instruction::RESET(ArithmeticTarget::E, 5)),
            0xAC => Some(Instruction::RESET(ArithmeticTarget::H, 5)),
            0xAD => Some(Instruction::RESET(ArithmeticTarget::L, 5)),
            0xAE => Some(Instruction::RESET(ArithmeticTarget::AddressHL, 5)),
            0xAF => Some(Instruction::RESET(ArithmeticTarget::A, 5)),

            // RESET instruction (bit 6)
            0xB0 => Some(Instruction::RESET(ArithmeticTarget::B, 6)),
            0xB1 => Some(Instruction::RESET(ArithmeticTarget::C, 6)),
            0xB2 => Some(Instruction::RESET(ArithmeticTarget::D, 6)),
            0xB3 => Some(Instruction::RESET(ArithmeticTarget::E, 6)),
            0xB4 => Some(Instruction::RESET(ArithmeticTarget::H, 6)),
            0xB5 => Some(Instruction::RESET(ArithmeticTarget::L, 6)),
            0xB6 => Some(Instruction::RESET(ArithmeticTarget::AddressHL, 6)),
            0xB7 => Some(Instruction::RESET(ArithmeticTarget::A, 6)),

            // RESET instruction (bit 7)
            0xB8 => Some(Instruction::RESET(ArithmeticTarget::B, 7)),
            0xB9 => Some(Instruction::RESET(ArithmeticTarget::C, 7)),
            0xBA => Some(Instruction::RESET(ArithmeticTarget::D, 7)),
            0xBB => Some(Instruction::RESET(ArithmeticTarget::E, 7)),
            0xBC => Some(Instruction::RESET(ArithmeticTarget::H, 7)),
            0xBD => Some(Instruction::RESET(ArithmeticTarget::L, 7)),
            0xBE => Some(Instruction::RESET(ArithmeticTarget::AddressHL, 7)),
            0xBF => Some(Instruction::RESET(ArithmeticTarget::A, 7)),

            // SET instruction (bit 0)
            0xC0 => Some(Instruction::SET(ArithmeticTarget::B, 0)),
            0xC1 => Some(Instruction::SET(ArithmeticTarget::C, 0)),
            0xC2 => Some(Instruction::SET(ArithmeticTarget::D, 0)),
            0xC3 => Some(Instruction::SET(ArithmeticTarget::E, 0)),
            0xC4 => Some(Instruction::SET(ArithmeticTarget::H, 0)),
            0xC5 => Some(Instruction::SET(ArithmeticTarget::L, 0)),
            0xC6 => Some(Instruction::SET(ArithmeticTarget::AddressHL, 0)),
            0xC7 => Some(Instruction::SET(ArithmeticTarget::A, 0)),

            // SET instruction (bit 1)
            0xC8 => Some(Instruction::SET(ArithmeticTarget::B, 1)),
            0xC9 => Some(Instruction::SET(ArithmeticTarget::C, 1)),
            0xCA => Some(Instruction::SET(ArithmeticTarget::D, 1)),
            0xCB => Some(Instruction::SET(ArithmeticTarget::E, 1)),
            0xCC => Some(Instruction::SET(ArithmeticTarget::H, 1)),
            0xCD => Some(Instruction::SET(ArithmeticTarget::L, 1)),
            0xCE => Some(Instruction::SET(ArithmeticTarget::AddressHL, 1)),
            0xCF => Some(Instruction::SET(ArithmeticTarget::A, 1)),

            // SET instruction (bit 2)
            0xD0 => Some(Instruction::SET(ArithmeticTarget::B, 2)),
            0xD1 => Some(Instruction::SET(ArithmeticTarget::C, 2)),
            0xD2 => Some(Instruction::SET(ArithmeticTarget::D, 2)),
            0xD3 => Some(Instruction::SET(ArithmeticTarget::E, 2)),
            0xD4 => Some(Instruction::SET(ArithmeticTarget::H, 2)),
            0xD5 => Some(Instruction::SET(ArithmeticTarget::L, 2)),
            0xD6 => Some(Instruction::SET(ArithmeticTarget::AddressHL, 2)),
            0xD7 => Some(Instruction::SET(ArithmeticTarget::A, 2)),

            // SET instruction (bit 3)
            0xD8 => Some(Instruction::SET(ArithmeticTarget::B, 3)),
            0xD9 => Some(Instruction::SET(ArithmeticTarget::C, 3)),
            0xDA => Some(Instruction::SET(ArithmeticTarget::D, 3)),
            0xDB => Some(Instruction::SET(ArithmeticTarget::E, 3)),
            0xDC => Some(Instruction::SET(ArithmeticTarget::H, 3)),
            0xDD => Some(Instruction::SET(ArithmeticTarget::L, 3)),
            0xDE => Some(Instruction::SET(ArithmeticTarget::AddressHL, 3)),
            0xDF => Some(Instruction::SET(ArithmeticTarget::A, 3)),

            // SET instruction (bit 4)
            0xE0 => Some(Instruction::SET(ArithmeticTarget::B, 4)),
            0xE1 => Some(Instruction::SET(ArithmeticTarget::C, 4)),
            0xE2 => Some(Instruction::SET(ArithmeticTarget::D, 4)),
            0xE3 => Some(Instruction::SET(ArithmeticTarget::E, 4)),
            0xE4 => Some(Instruction::SET(ArithmeticTarget::H, 4)),
            0xE5 => Some(Instruction::SET(ArithmeticTarget::L, 4)),
            0xE6 => Some(Instruction::SET(ArithmeticTarget::AddressHL, 4)),
            0xE7 => Some(Instruction::SET(ArithmeticTarget::A, 4)),

            // SET instruction (bit 5)
            0xE8 => Some(Instruction::SET(ArithmeticTarget::B, 5)),
            0xE9 => Some(Instruction::SET(ArithmeticTarget::C, 5)),
            0xEA => Some(Instruction::SET(ArithmeticTarget::D, 5)),
            0xEB => Some(Instruction::SET(ArithmeticTarget::E, 5)),
            0xEC => Some(Instruction::SET(ArithmeticTarget::H, 5)),
            0xED => Some(Instruction::SET(ArithmeticTarget::L, 5)),
            0xEE => Some(Instruction::SET(ArithmeticTarget::AddressHL, 5)),
            0xEF => Some(Instruction::SET(ArithmeticTarget::A, 5)),

            // SET instruction (bit 6)
            0xF0 => Some(Instruction::SET(ArithmeticTarget::B, 6)),
            0xF1 => Some(Instruction::SET(ArithmeticTarget::C, 6)),
            0xF2 => Some(Instruction::SET(ArithmeticTarget::D, 6)),
            0xF3 => Some(Instruction::SET(ArithmeticTarget::E, 6)),
            0xF4 => Some(Instruction::SET(ArithmeticTarget::H, 6)),
            0xF5 => Some(Instruction::SET(ArithmeticTarget::L, 6)),
            0xF6 => Some(Instruction::SET(ArithmeticTarget::AddressHL, 6)),
            0xF7 => Some(Instruction::SET(ArithmeticTarget::A, 6)),

            // SET instruction (bit 7)
            0xF8 => Some(Instruction::SET(ArithmeticTarget::B, 7)),
            0xF9 => Some(Instruction::SET(ArithmeticTarget::C, 7)),
            0xFA => Some(Instruction::SET(ArithmeticTarget::D, 7)),
            0xFB => Some(Instruction::SET(ArithmeticTarget::E, 7)),
            0xFC => Some(Instruction::SET(ArithmeticTarget::H, 7)),
            0xFD => Some(Instruction::SET(ArithmeticTarget::L, 7)),
            0xFE => Some(Instruction::SET(ArithmeticTarget::AddressHL, 7)),
            0xFF => Some(Instruction::SET(ArithmeticTarget::A, 7)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),
            0x01 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::BC, LoadWordSource::D16))),
            0x02 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::A))),
            0x03 => Some(Instruction::INC(ArithmeticTarget::BC)),
            0x04 => Some(Instruction::INC(ArithmeticTarget::B)),
            0x05 => Some(Instruction::DEC(ArithmeticTarget::B)),
            0x06 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),
            0x07 => Some(Instruction::RLCA),
            0x08 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::Address16, LoadWordSource::SP))),
            0x09 => Some(Instruction::ADDHL(ArithmeticTarget::BC)),
            0x0A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressBC))),
            0x0B => Some(Instruction::DEC(ArithmeticTarget::BC)),
            0x0C => Some(Instruction::INC(ArithmeticTarget::C)),
            0x0D => Some(Instruction::DEC(ArithmeticTarget::C)),
            0x0E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),
            0x0F => Some(Instruction::RRCA),

            0x10 => Some(Instruction::STOP),
            0x11 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16))),
            0x12 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressDE, LoadByteSource::A))),
            0x13 => Some(Instruction::INC(ArithmeticTarget::DE)),
            0x14 => Some(Instruction::INC(ArithmeticTarget::D)),
            0x15 => Some(Instruction::DEC(ArithmeticTarget::D)),
            0x16 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),
            0x17 => Some(Instruction::RLA),
            0x18 => Some(Instruction::JR(JumpTest::Always)), // Remplacez "offset" par la valeur appropriée.
            0x19 => Some(Instruction::ADDHL(ArithmeticTarget::DE)),
            0x1A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressDE))),
            0x1B => Some(Instruction::DEC(ArithmeticTarget::DE)),
            0x1C => Some(Instruction::INC(ArithmeticTarget::E)),
            0x1D => Some(Instruction::DEC(ArithmeticTarget::E)),
            0x1E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),
            0x1F => Some(Instruction::RRA),

            0x20 => Some(Instruction::JR(JumpTest::NotZero)), // Remplacez "offset" par la valeur appropriée.
            0x21 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::D16))),
            0x22 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHLP, LoadByteSource::A))),
            0x23 => Some(Instruction::INC(ArithmeticTarget::HL)),
            0x24 => Some(Instruction::INC(ArithmeticTarget::H)),
            0x25 => Some(Instruction::DEC(ArithmeticTarget::H)),
            0x26 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),
            0x27 => Some(Instruction::DAA),
            0x28 => Some(Instruction::JR(JumpTest::Zero)), // Remplacez "offset" par la valeur appropriée.
            0x29 => Some(Instruction::ADDHL(ArithmeticTarget::HL)),
            0x2A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressHLP))),
            0x2B => Some(Instruction::DEC(ArithmeticTarget::HL)),
            0x2C => Some(Instruction::INC(ArithmeticTarget::L)),
            0x2D => Some(Instruction::DEC(ArithmeticTarget::L)),
            0x2E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),
            0x2F => Some(Instruction::CPL),

            0x30 => Some(Instruction::JR(JumpTest::NotCarry)), // Remplacez "offset" par la valeur appropriée.
            0x31 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::D16))),
            0x32 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHLM, LoadByteSource::A))),
            0x33 => Some(Instruction::INC(ArithmeticTarget::SP)),
            0x34 => Some(Instruction::INC(ArithmeticTarget::AddressHL)),
            0x35 => Some(Instruction::DEC(ArithmeticTarget::AddressHL)),
            0x36 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::D8))),
            0x37 => Some(Instruction::SCF),
            0x38 => Some(Instruction::JR(JumpTest::Carry)), // Remplacez "offset" par la valeur appropriée.
            0x39 => Some(Instruction::ADDHL(ArithmeticTarget::SP)),
            0x3A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressHLM))),
            0x3B => Some(Instruction::DEC(ArithmeticTarget::SP)),
            0x3C => Some(Instruction::INC(ArithmeticTarget::A)),
            0x3D => Some(Instruction::DEC(ArithmeticTarget::A)),
            0x3E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),
            0x3F => Some(Instruction::CCF),

            0x40 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),
            0x41 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),
            0x42 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),
            0x43 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),
            0x44 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),
            0x45 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),
            0x46 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::AddressHLM))),
            0x47 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),
            0x48 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),
            0x49 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),
            0x4A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),
            0x4B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),
            0x4C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))),
            0x4D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),
            0x4E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::AddressHLM))),
            0x4F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),

            0x50 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),
            0x51 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),
            0x52 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),
            0x53 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),
            0x54 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),
            0x55 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),
            0x56 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::AddressHL))),
            0x57 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),
            0x58 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),
            0x59 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),
            0x5A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),
            0x5B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),
            0x5C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),
            0x5D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),
            0x5E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::AddressHL))),
            0x5F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),

            0x60 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),
            0x61 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),
            0x62 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))),
            0x63 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))),
            0x64 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),
            0x65 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),
            0x66 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::AddressHL))),
            0x67 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),
            0x68 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),
            0x69 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),
            0x6A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),
            0x6B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),
            0x6C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),
            0x6D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),
            0x6E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::AddressHL))),
            0x6F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),

            0x70 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::B))),
            0x71 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::C))),
            0x72 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::D))),
            0x73 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::E))),
            0x74 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::H))),
            0x75 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::L))),
            0x76 => Some(Instruction::HALT),
            0x77 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::A))),
            0x78 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),
            0x79 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),
            0x7A => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),
            0x7B => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),
            0x7C => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),
            0x7D => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),
            0x7E => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressHL))),
            0x7F => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),

            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::AddressHL)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArithmeticTarget::AddressHL)),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),

            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::AddressHL)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8A => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8B => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8C => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8D => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8E => Some(Instruction::ADC(ArithmeticTarget::AddressHL)),
            0x8F => Some(Instruction::ADC(ArithmeticTarget::A)),

            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::AddressHL)),
            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9A => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9B => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9C => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9D => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9E => Some(Instruction::SBC(ArithmeticTarget::AddressHL)),
            0x9F => Some(Instruction::SBC(ArithmeticTarget::A)),

            0xA0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xA1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xA2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xA3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xA4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xA5 => Some(Instruction::AND(ArithmeticTarget::L)),
            0xA6 => Some(Instruction::AND(ArithmeticTarget::AddressHL)),
            0xA7 => Some(Instruction::AND(ArithmeticTarget::A)),
            0xA8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xA9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xAA => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xAB => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xAC => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xAD => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xAE => Some(Instruction::XOR(ArithmeticTarget::AddressHL)),
            0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),

            0xB0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xB1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xB2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xB3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xB4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xB5 => Some(Instruction::OR(ArithmeticTarget::L)),
            0xB6 => Some(Instruction::OR(ArithmeticTarget::AddressHL)),
            0xB7 => Some(Instruction::OR(ArithmeticTarget::A)),
            0xB8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xB9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xBA => Some(Instruction::CP(ArithmeticTarget::D)),
            0xBB => Some(Instruction::CP(ArithmeticTarget::E)),
            0xBC => Some(Instruction::CP(ArithmeticTarget::H)),
            0xBD => Some(Instruction::CP(ArithmeticTarget::L)),
            0xBE => Some(Instruction::CP(ArithmeticTarget::AddressHL)),
            0xBF => Some(Instruction::CP(ArithmeticTarget::A)),

            0xC0 => Some(Instruction::RET(JumpTest::NotZero)),
            0xC1 => Some(Instruction::POP(StackTarget::BC)),
            0xC2 => Some(Instruction::JP(JumpTest::NotZero, JumpCondition::Address16)),
            0xC3 => Some(Instruction::JP(JumpTest::Always, JumpCondition::Address16)),
            0xC4 => Some(Instruction::CALL(JumpTest::NotZero, JumpCondition::Address16)),
            0xC5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xC6 => Some(Instruction::ADD(ArithmeticTarget::D8)),
            0xC7 => Some(Instruction::RST(RstTarget::Rst00H)),
            0xC8 => Some(Instruction::RET(JumpTest::Zero)),
            0xC9 => Some(Instruction::RET(JumpTest::Always)),
            0xCA => Some(Instruction::JP(JumpTest::Zero, JumpCondition::Address16)),
            0xCB => Some(Instruction::PrefixCB),
            0xCC => Some(Instruction::CALL(JumpTest::Zero, JumpCondition::Address16)),
            0xCD => Some(Instruction::CALL(JumpTest::Always, JumpCondition::Address16)),
            0xCE => Some(Instruction::ADC(ArithmeticTarget::D8)),
            0xCF => Some(Instruction::RST(RstTarget::Rst08H)),

            0xD0 => Some(Instruction::RET(JumpTest::NotCarry)),
            0xD1 => Some(Instruction::POP(StackTarget::DE)),
            0xD2 => Some(Instruction::JP(JumpTest::NotCarry, JumpCondition::Address16)),
            0xD4 => Some(Instruction::CALL(JumpTest::NotCarry, JumpCondition::Address16)),
            0xD5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xD6 => Some(Instruction::SUB(ArithmeticTarget::D8)),
            0xD7 => Some(Instruction::RST(RstTarget::Rst10H)),
            0xD8 => Some(Instruction::RET(JumpTest::Carry)),
            0xD9 => Some(Instruction::RETI),
            0xDA => Some(Instruction::JP(JumpTest::Carry, JumpCondition::Address16)),
            0xDC => Some(Instruction::CALL(JumpTest::Carry, JumpCondition::Address16)),
            0xDE => Some(Instruction::SBC(ArithmeticTarget::D8)),
            0xDF => Some(Instruction::RST(RstTarget::Rst18H)),

            0xE0 => Some(Instruction::LDH(LoadType::LoadByte(LoadByteTarget::Address8, LoadByteSource::A))),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0xE2 => Some(Instruction::LD(LoadType::LoadByte(LoadByteTarget::AddressC, LoadByteSource::A))),
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xE6 => Some(Instruction::AND(ArithmeticTarget::D8)),
            0xE7 => Some(Instruction::RST(RstTarget::Rst20H)),
            0xE8 => Some(Instruction::ADDSP(ArithmeticTarget::D8)),
            0xE9 => Some(Instruction::JP(JumpTest::Always, JumpCondition::HL)),
            0xEA => Some(Instruction::LD(LoadType::LoadByte(LoadByteTarget::Address16, LoadByteSource::A))),
            0xEE => Some(Instruction::XOR(ArithmeticTarget::D8)),
            0xEF => Some(Instruction::RST(RstTarget::Rst28H)),

            0xF0 => Some(Instruction::LDH(LoadType::LoadByte(LoadByteTarget::A, LoadByteSource::Address8))),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),
            0xF2 => Some(Instruction::LD(LoadType::LoadByte(LoadByteTarget::A, LoadByteSource::AddressC))),
            0xF3 => Some(Instruction::DI),
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),
            0xF6 => Some(Instruction::OR(ArithmeticTarget::D8)),
            0xF7 => Some(Instruction::RST(RstTarget::Rst30H)),
            0xF8 => Some(Instruction::LD(LoadType::LoadHLSP)),
            0xF9 => Some(Instruction::LD(LoadType::LoadSPHL)),
            0xFA => Some(Instruction::LD(LoadType::LoadByte(LoadByteTarget::A, LoadByteSource::MemoryAddress16))),
            0xFB => Some(Instruction::EI),
            0xFE => Some(Instruction::CP(ArithmeticTarget::D8)),
            0xFF => Some(Instruction::RST(RstTarget::Rst38H)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
}
impl CPU {

    fn read_next_byte(&self) -> u8 {
        let byte = self.bus.read_byte(self.pc);
        self.pc += 1;
        byte
    }

    fn read_next_word(&self) -> u16 {
        let word = self.bus.read_word(self.pc);
        self.pc += 2;
        word
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::AddressHL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => todo!()

                }
            }

            Instruction::ADDHL(target) => self.execute_add_hl(target),

            Instruction::ADC(target) => self.execute_adc(target),

            Instruction::SUB(target) => self.execute_sub(target),

            Instruction::SBC(target) => self.execute_sbc(target),

            Instruction::AND(target) => self.execute_and(target),

            Instruction::OR(target) => self.execute_or(target),

            Instruction::XOR(target) => self.execute_xor(target),

            Instruction::CP(target) => self.execute_cp(target),

            Instruction::INC(target) => {
                match target {
                    ArithmeticTarget::A | ArithmeticTarget::B | ArithmeticTarget::C | ArithmeticTarget::D
                    | ArithmeticTarget::E | ArithmeticTarget::H | ArithmeticTarget::L | ArithmeticTarget::AddressHL => {
                        self.execute_inc(target);
                    }
                    //pas de modifications de flag
                    ArithmeticTarget::BC => {
                        let value = self.registers.get_bc();
                        let new_value = value.wrapping_add(1);
                        self.registers.set_bc() = new_value;
                    }
                    ArithmeticTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = value.wrapping_add(1);
                        self.registers.set_de() = new_value;
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = value.wrapping_add(1);
                        self.registers.set_hl() = new_value;
                    }
                    ArithmeticTarget::SP => {
                        let value = self.sp;
                        let new_value = value.wrapping_add(1);
                        self.sp = new_value;
                    }
                    _ => todo!()
                }
            },

            Instruction::DEC(target) => {
                match target {
                    ArithmeticTarget::A | ArithmeticTarget::B | ArithmeticTarget::C | ArithmeticTarget::D
                    | ArithmeticTarget::E | ArithmeticTarget::H | ArithmeticTarget::L | ArithmeticTarget::AddressHL => {
                        self.execute_dec(target);
                    }
                    //pas de modifications de flag
                    ArithmeticTarget::BC => {
                        let value = self.registers.get_bc();
                        let new_value = value.wrapping_sub(1);
                        self.registers.set_bc() = new_value;
                    }
                    ArithmeticTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = value.wrapping_sub(1);
                        self.registers.set_de() = new_value;
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = value.wrapping_sub(1);
                        self.registers.set_hl() = new_value;
                    }
                    ArithmeticTarget::SP => {
                        let value = self.sp;
                        let new_value = value.wrapping_sub(1);
                        self.sp = new_value;
                    }
                    _ => todo!()
                }
            },

            Instruction::CCF => self.execute_ccf(),

            Instruction::SCF => self.execute_scf(),

            Instruction::CPL => self.execute_cpl(),

            Instruction::RRA => self.execute_rra(),

            Instruction::RLA => self.execute_rla(),

            Instruction::RRCA => self.execute_rrca(),

            Instruction::RLCA => self.execute_rlca(),

            Instruction::BIT(target, bit_num) => self.execute_bit(target, bit_num),

            Instruction::RESET(target, bit_num) => self.execute_reset(target, bit_num),

            Instruction::SET(target, bit_num) => self.execute_set(target, bit_num),

            Instruction::SRL(target) => self.execute_srl(target),

            Instruction::RR(target) => self.execute_rr(target),

            Instruction::RL(target) => self.execute_rl(target),

            Instruction::RRC(target) => self.execute_rrc(target),

            Instruction::RLC(target) => self.execute_rlc(target),

            Instruction::SRA(target) => self.execute_sra(target),

            Instruction::SLA(target) => self.execute_sla(target),

            Instruction::SWAP(target) => self.execute_swap(target),

            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {

                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::AddressBC => self.bus.read_byte(self.registers.get_bc()),
                            LoadByteSource::AddressDE => self.bus.read_byte(self.registers.get_de()),
                            LoadByteSource::AddressHLP => self.bus.read_byte(self.registers.get_hlp()),
                            LoadByteSource::AddressHLM => self.bus.read_byte(self.registers.get_hlm()),
                            LoadByteSource::AddressC => self.bus.read_byte(0xFF00 | (self.registers.c as u16)),
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::AddressHL => self.bus.read_byte(self.registers.get_hl()),
                            _ => { panic!("TODO: implement other sources") }
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                            LoadByteTarget::AddressHL => self.bus.write_byte(self.registers.get_hl(), source_value),
                            LoadByteTarget::AddressBC => self.bus.write_byte(self.registers.get_bc(), source_value),
                            LoadByteTarget::AddressDE => self.bus.write_byte(self.registers.get_de(), source_value),
                            LoadByteTarget::AddressHLP =>self.bus.write_byte(self.registers.get_hlp(), source_value),
                            LoadByteTarget::AddressHLM => self.bus.write_byte(self.registers.get_hlm(), source_value),
                            LoadByteTarget::AddressC => self.bus.write_byte(0xFF00 | (self.registers.c as u16) , source_value),
                            _ => { panic!("TODO: implement other targets") }
                        };
                        match (source, target) {
                            (LoadByteSource::D8, _)
                            | (LoadByteSource::A, LoadByteTarget::AddressC)
                            | (LoadByteSource::AddressC, LoadByteTarget::A) => {
                                self.pc = self.pc.wrapping_add(2);
                            }
                            _ => {
                                self.pc = self.pc.wrapping_add(1);
                            }
                        }
                    }
                    _ => { panic!("TODO: implement other load types") }
                }
            }

            Instruction::LDH(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {

                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::Address8 => self.bus.read_byte(0xFF00 | (self.read_next_byte() as u16)),
                            _ => { panic!("TODO: implement other sources") }
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::Address8 => self.bus.write_byte(0xFF00 | (self.read_next_byte() as u16) , source_value),
                            _ => { panic!("TODO: implement other targets") }
                        };
                        self.pc = self.pc.wrapping_add(1);
                    }
                    _ => { panic!("TODO: implement other load types") }
                }
            }

            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    StackTarget::AF => self.registers.get_af(),

                    _ => { panic!("TODO: support more targets") }
                };
                self.push(value);
                self.pc.wrapping_add(1);
            }

            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                    StackTarget::AF => self.registers.set_af(result),
                    _ => { panic!("TODO: support more targets") }
                };
                self.pc.wrapping_add(1);
            }

            Instruction::CALL(test, target) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    _ => { panic!("TODO: support more conditions") }
                };
                self.call(jump_condition,target);
            }

            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    _ => { panic!("TODO: support more conditions") }
                };
                self.return_(jump_condition);
            }

            Instruction::DAA => {self.daa();}

            Instruction::JP(test, ju) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump(jump_condition,ju);
            }

            Instruction::JR(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump_relative(jump_condition);
            }

            Instruction::JPI => self.jump_indirect(),

            Instruction::RETI => {
                self.pc = self.pop();
                self.ei = 1;
            },

            Instruction::STOP => 1,

            Instruction::HALT =>{self.halt = true;
                1},

            Instruction::EI => {
                self.ei  = 2;
            },

            Instruction::DI => {
                self.di  = 1;
            },

            Instruction::NOP => 1,

            Instruction::RST(npc) => {
                let old_pc = self.pc;
                self.push(old_pc);
                match npc {
                    RstTarget::Rst00H => self.pc = 0x00,
                    RstTarget::Rst08H => self.pc = 0x08,
                    RstTarget::Rst10H => self.pc = 0x10,
                    RstTarget::Rst18H => self.pc = 0x18,
                    RstTarget::Rst20H => self.pc = 0x20,
                    RstTarget::Rst28H => self.pc = 0x28,
                    RstTarget::Rst30H => self.pc = 0x30,
                    RstTarget::Rst38H => self.pc = 0x38,
                }
            },

            Instruction::ADDSP(targetd8) => execute_addsp(targetd8),
        }
    }

    fn daa(&mut self) {
        let mut result = self.registers.a as u16;

        if self.registers.f.subtract == false {
            if self.registers.f.carry || result > 0x99 {
                result += 0x60;
                self.registers.f.carry = true;
            }
            if self.registers.f.half_carry || (self.registers.a & 0x0F) > 0x09 {
                result += 0x06;
            }
        } else {
            if self.registers.f.carry {
                result -= 0x60;
            }
            if self.registers.f.half_carry {
                result -= 0x06;
            }
        }

        self.registers.a = (result & 0xFF) as u8;

        self.registers.f.zero = self.registers.a == 0;
        self.registers.f.half_carry = false; // Reset half carry flag.
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

    fn execute_add_hl(&mut self, target: ArithmeticTarget) {
        let hl_value = self.registers.get_hl();
        let value = match target {
            ArithmeticTarget::BC => self.registers.get_bc(),
            ArithmeticTarget::DE => self.registers.get_de(),
            ArithmeticTarget::HL => self.registers.get_hl(),
            ArithmeticTarget::SP => self.sp,
            _ => todo!()

        };
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower 12 bits of the value and register HL
        // together result in a value bigger than 0xFFF. If the result is larger than 0xFFF,
        // then the addition caused a carry from the lower 12 bits to the upper 4 bits.
        self.registers.f.half_carry = (self.registers.get_hl() & 0xFFF) + (value & 0xFFF) > 0xFFF;
        self.registers.set_hl(new_value);
    }

    fn execute_addsp(&mut self) {
        let signed_byte = self.read_next_byte() as i8;
        let value = self.sp;
        let (new_value, did_overflow) = self.sp.overflowing_add(signed_byte);
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower 12 bits of the value and register HL
        // together result in a value bigger than 0xFFF. If the result is larger than 0xFFF,
        // then the addition caused a carry from the lower 12 bits to the upper 4 bits.
        self.registers.f.half_carry = (self.registers.get_hl() & 0xFFF) + (value & 0xFFF) > 0xFFF;
        self.sp=new_value;
    }

    fn execute_adc(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => self.bus.read_byte(self.registers.get_hl()),
            ArithmeticTarget::D8 => self.read_next_byte(),
            _=> todo!(),
        };

        let carry = if self.registers.f.carry { 1 } else { 0 };

        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        //Add the carry to the new value
        let (new_value_with_carry, did_carry_overflow) = new_value.overflowing_add(carry);


        self.registers.f.zero = new_value_with_carry == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow || did_carry_overflow;
        self.registers.f.half_carry = (self.registers.a & 0x0F) + (value & 0x0F) + carry > 0x0F;

        self.registers.a = new_value_with_carry;
    }

    fn execute_sub(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => self.bus.read_byte(self.registers.get_hl()),
            ArithmeticTarget::D8 => self.read_next_byte(),
            _ => todo!(),

        };
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F);

        self.registers.a = new_value;
    }

    fn execute_sbc(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => self.bus.read_byte(self.registers.get_hl()),
            ArithmeticTarget::D8 => self.read_next_byte(),
            _ => todo!(),

        };

        let carry_bit = if self.registers.f.carry { 1 } else { 0 }; // Get the carry flag as a 0 or 1.
        let (new_value, did_underflow) = self.registers.a.overflowing_sub(value);
        let (with_carry, carry_underflow) = new_value.overflowing_sub(carry_bit);

        // Check if there is a borrow from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F) + carry_bit;

        self.registers.f.zero = with_carry == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_underflow || carry_underflow;

        self.registers.a = with_carry;
    }

    fn execute_and(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => self.bus.read_byte(self.registers.get_hl()),
            ArithmeticTarget::D8 => self.read_next_byte(),
            _ => todo!(),

        };
        let result = self.registers.a & value;

        self.registers.a = result;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true; // Half Carry est toujours défini sur true dans l'opération AND.
        self.registers.f.carry = false; // Carry est toujours défini sur false dans l'opération AND.
    }

    fn execute_or(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => self.bus.read_byte(self.registers.get_hl()),
            ArithmeticTarget::D8 => self.read_next_byte(),
            _ => todo!(),

        };

        let result = self.registers.a | value;

        self.registers.a = result;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false; // Half Carry est toujours défini sur false dans l'opération OU.
        self.registers.f.carry = false; // Carry est toujours défini sur false dans l'opération OU.
    }

    fn execute_xor(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => self.bus.read_byte(self.registers.get_hl()),
            ArithmeticTarget::D8 => self.read_next_byte(),
            _ => todo!(),
        };

        let result = self.registers.a ^ value;

        self.registers.a = result;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false; // Half Carry est toujours défini sur false dans l'opération XOR.
        self.registers.f.carry = false; // Carry est toujours défini sur false dans l'opération XOR.
    }

    fn execute_cp(&mut self, target: ArithmeticTarget) {
        let value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => self.bus.read_byte(self.registers.get_hl()),
            ArithmeticTarget::D8 => self.read_next_byte(),
            _ => todo!(),
            //AddressHL -> gethl read address dans memory
            //d8 readnextbyte
        };

        let (_result, did_underflow) = self.registers.a.overflowing_sub(value);

        // Le résultat de la soustraction n'est pas stocké, seulement les drapeaux sont mis à jour.
        self.registers.f.zero = did_underflow;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F);
        self.registers.f.carry = did_underflow;
    }

    fn execute_inc(&mut self, target: ArithmeticTarget) {
        let new_value = match target {
            ArithmeticTarget::A => {
                let value = self.registers.a;
                let new_value = self.registers.a.wrapping_add(1);
                self.registers.a = new_value;
                new_value
            }
            ArithmeticTarget::B => {
                let value = self.registers.b;
                let new_value = self.registers.b.wrapping_add(1);
                self.registers.b = new_value;
                new_value
            }
            ArithmeticTarget::C => {
                let value = self.registers.c;
                let new_value = self.registers.c.wrapping_add(1);
                self.registers.c = new_value;
                new_value
            }
            ArithmeticTarget::D => {
                let value = self.registers.d;
                let new_value = self.registers.d.wrapping_add(1);
                self.registers.d = new_value;
                new_value
            }
            ArithmeticTarget::E => {
                let value = self.registers.e;
                let new_value = self.registers.e.wrapping_add(1);
                self.registers.e = new_value;
                new_value
            }
            ArithmeticTarget::H => {
                let value = self.registers.h;
                let new_value = self.registers.h.wrapping_add(1);
                self.registers.h = new_value;
                new_value
            }
            ArithmeticTarget::L => {
                let value = self.registers.l;
                let new_value = self.registers.l.wrapping_add(1);
                self.registers.l = new_value;
                new_value
            }
            ArithmeticTarget::AddressHL => {
                let address=self.registers.get_hl();
                let value = self.bus.read_byte(address);
                let new_value = value.wrapping_add(1);
                self.bus.write_byte(address, new_value);
                new_value
            }
            _ => todo!()
        };

        // Mettez à jour les drapeaux appropriés.
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (new_value & 0x0F) == 0;
    }

    fn execute_dec(&mut self, target: ArithmeticTarget) {
        let new_value = match target {
            ArithmeticTarget::A => {
                let value = self.registers.a;
                let new_value = self.registers.a.wrapping_sub(1);
                self.registers.a = new_value;
                new_value
            }
            ArithmeticTarget::B => {
                let value = self.registers.b;
                let new_value = self.registers.b.wrapping_sub(1);
                self.registers.b = new_value;
                new_value
            }
            ArithmeticTarget::C => {
                let value = self.registers.c;
                let new_value = self.registers.c.wrapping_sub(1);
                self.registers.c = new_value;
                new_value
            }
            ArithmeticTarget::D => {
                let value = self.registers.d;
                let new_value = self.registers.d.wrapping_sub(1);
                self.registers.d = new_value;
                new_value
            }
            ArithmeticTarget::E => {
                let value = self.registers.e;
                let new_value = self.registers.e.wrapping_sub(1);
                self.registers.e = new_value;
                new_value
            }
            ArithmeticTarget::H => {
                let value = self.registers.h;
                let new_value = self.registers.h.wrapping_sub(1);
                self.registers.h = new_value;
                new_value
            }
            ArithmeticTarget::L => {
                let value = self.registers.l;
                let new_value = self.registers.l.wrapping_sub(1);
                self.registers.l = new_value;
                new_value
            }
            ArithmeticTarget::AddressHL => {
                let address=self.registers.get_hl();
                let value = self.bus.read_byte(address);
                let new_value = value.wrapping_sub(1);
                self.bus.write_byte(address, new_value);
                new_value
            }
            _ => todo!()
        };

        // Mettez à jour les drapeaux appropriés (Zéro, Soustraction, Demi-retenue).
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (new_value & 0x0F) == 0x0F;
    }

    fn execute_ccf(&mut self) {
        // Bascule la valeur du carry flag.
        self.registers.f.carry = !self.registers.f.carry;

        // Mettez à jour les autres drapeaux (subtract et half_carry) comme spécifié dans la documentation.
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    fn execute_scf(&mut self) {
        // Définir le drapeau de retenue (carry flag) sur vrai.
        self.registers.f.carry = true;

        // Réinitialiser les drapeaux N et H.
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = true;
    }

    fn execute_cpl(&mut self) {
        // Inversion de chaque bit du registre A.
        self.registers.a = !self.registers.a;

        // Réglage des drapeaux appropriés.
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
    }

    fn execute_rra(&mut self) {
        let carry_flag = self.registers.f.carry;
        let bit0 = self.registers.a & 0x01;

        self.registers.a >>= 1;

        if carry_flag {
            self.registers.a |= 0x80;
        }

        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = bit0 != 0;
    }

    fn execute_rla(&mut self) {
        let carry_flag = self.registers.f.carry;
        let bit7 = (self.registers.a & 0x80) != 0;

        self.registers.a <<= 1;

        if carry_flag {
            self.registers.a |= 0x01;
        }

        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = bit7;
    }

    fn execute_rrca(&mut self) {
        let bit0 = self.registers.a & 0x01;

        self.registers.a >>= 1;

        // Le bit retenu est maintenant le bit0.
        self.registers.f.carry = bit0 != 0;

        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    fn execute_rlca(&mut self) {
        let bit7 = (self.registers.a & 0x80) != 0;

        self.registers.a <<= 1;

        // Le bit retenu est maintenant le bit7.
        self.registers.f.carry = bit7;

        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    fn execute_bit(&mut self, target: ArithmeticTarget, bit_num: u8) {
        let value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // You may need to implement memory read here.
            },
            _ => todo!(),
        };

        // Test the specified bit and set flags accordingly.
        let mask = 1 << bit_num;
        let bit_set = (value & mask) != 0;
        self.registers.f.zero = !bit_set;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
    }

    fn execute_reset(&mut self, target: ArithmeticTarget, bit_num: u8) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => todo!(),
        };

        // Reset the specified bit.
        let mask = !(1 << bit_num);
        value &= mask;

        // Update the register or memory with the new value.
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => todo!(),
        }
    }

    fn execute_set(&mut self, target: ArithmeticTarget, bit_num: u8) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => todo!(),
        };

        // Set the specified bit.
        let mask = 1 << bit_num;
        value |= mask;

        // Update the register or memory with the new value.
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => todo!(),
        }
    }

    fn execute_srl(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory
            },
            _ => todo!(),
        };

        // Perform the bitwise right shift.
        let carry = value & 0x01 != 0; // Store the carry bit.

        value >>= 1; // Right shift by 1 bit.

        // Update the register or memory with the new value.
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => todo!()
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    fn execute_rr(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => todo!(),
        };

        // Extract the carry bit and store it.
        let carry = value & 0x01 != 0;

        // Perform the bitwise right rotation.
        value >>= 1;
        if self.registers.f.carry {
            value |= 0x80; // Set the leftmost bit to the previous carry value.
        }

        // Update the register or memory with the new value.
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => todo!(),
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    fn execute_rl(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => todo!(),
        };

        // Extract the carry bit and store it.
        let carry = (value & 0x80) != 0;

        // Perform the bitwise left rotation.
        value <<= 1;
        if self.registers.f.carry {
            value |= 0x01; // Set the rightmost bit to the previous carry value.
        }

        // Update the register or memory with the new value.
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => todo!(),
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    fn execute_rrc(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => todo!(),
        };

        // Extract the rightmost bit and store it as the new carry.
        let carry = (value & 0x01) != 0;

        // Perform the bitwise right rotation.
        value >>= 1;
        if carry {
            value |= 0x80; // Set the leftmost bit to the previous carry value.
        }

        // Update the register or memory with the new value.
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => todo!(),
        }

        // Update the flags.
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    fn execute_rlc(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => todo!(),
        };

        // Extract the leftmost bit and store it as the new carry.
        let carry = (value & 0x80) != 0;

        // Perform the bitwise left rotation.
        value <<= 1;
        if carry {
            value |= 0x01; // Set the rightmost bit to the previous carry value.
        }

        // Update the register or memory with the new value.
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => todo!(),
        }

        // Update the flags.
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    fn execute_sra(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => todo!(),
        };

        // Extract the rightmost bit and store it as the new carry.
        let carry = value & 0x01 != 0;

        // Perform the arithmetic right shift.
        let msb = value & 0x80;
        value = (value >> 1) | msb; // Fill the leftmost bit with the previous MSB.

        // Update the register or memory with the new value.
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => todo!(),
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    fn execute_sla(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => todo!(),
        };

        // Extract the leftmost bit and store it as the new carry.
        let carry = (value & 0x80) != 0;

        // Perform the arithmetic left shift.
        value <<= 1;

        // Update the register or memory with the new value.
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => todo!(),
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    fn execute_swap(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => todo!(),
        };

        // Perform the swap of the upper and lower nibbles.
        let lower_nibble = value & 0x0F;
        let upper_nibble = (value & 0xF0) >> 4;
        value = (lower_nibble << 4) | upper_nibble;

        // Update the register or memory with the new value.
        match target {
            ArithmeticTarget::A => self.registers.a = value,
            ArithmeticTarget::B => self.registers.b = value,
            ArithmeticTarget::C => self.registers.c = value,
            ArithmeticTarget::D => self.registers.d = value,
            ArithmeticTarget::E => self.registers.e = value,
            ArithmeticTarget::H => self.registers.h = value,
            ArithmeticTarget::L => self.registers.l = value,
            ArithmeticTarget::HL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => todo!(),
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false; // Clear the carry flag.
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }



    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };
    }

    fn jump(&self, should_jump: bool, ju : JumpCondition) -> u16 {
        match ju {
            JumpCondition::Address16 => {
                if should_jump {
                    let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
                    let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
                    (most_significant_byte << 8) | least_significant_byte
                } else {
                    self.pc.wrapping_add(3)
                }
            },
            JumpCondition::AddressHL => {
                self.registers.get_hl()
            }
        }
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn jump_relative(&mut self, condition: bool) {
        if condition {
            let new_pc = ((self.pc as i32) + self.read_next_byte() as i32) as u16;
            self.pc = new_pc;
        } else {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    fn jump_indirect(&mut self) {
        let address = self.registers.get_hl(); // Obtenir la valeur de HL (adresse à sauter)
        self.pc = address; // Copier l'adresse dans le PC
    }
    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }
}

