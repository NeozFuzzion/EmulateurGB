pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

#[derive(Debug)]
pub enum JumpCondition{
    Address16, AddressHL
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource)
}

pub enum LoadByteTarget{
    A, B, C, D, E, H, L, AddressHL, AddressDE, AddressHLP, AddressHLM, AddressC, Address16, Address8
}

pub enum LoadByteSource{
    A, B, C, D, E, H, L, AddressHL, AddressBC, AddressDE, AddressHLP, AddressHLM, AddressC, Address16, D8, Address8
}

pub enum LoadWordTarget{
    BC, DE, HL, SP, Address16
}

pub enum LoadWordSource{
    D16, SP, HL, SPR8
}

pub enum StackTarget {AF, HL , BC, DE}


pub enum RstTarget{
    Rst00H, Rst08H, Rst10H, Rst18H, Rst20H, Rst28H, Rst30H, Rst38H,
}

pub enum Instruction {
    NOP,
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADDSP(),
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
    CALL(JumpTest),
    RET(JumpTest),
    JR(JumpTest),
    RETI,
    STOP,
    HALT,
    EI,
    DI,
    RST(RstTarget),
    PrefixCB
}

#[derive(Debug)]
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L, HL, BC, DE, SP, AddressHL, D8
}


impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> (Option<Instruction>,u8) {
        
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    pub fn from_byte_prefixed(byte: u8) -> (Option<Instruction>,u8) {
        match byte {
            0x00 => (Some(Instruction::RLC(ArithmeticTarget::B)),2),
            0x01 => (Some(Instruction::RLC(ArithmeticTarget::C)),2),
            0x02 => (Some(Instruction::RLC(ArithmeticTarget::D)),2),
            0x03 => (Some(Instruction::RLC(ArithmeticTarget::E)),2),
            0x04 => (Some(Instruction::RLC(ArithmeticTarget::H)),2),
            0x05 => (Some(Instruction::RLC(ArithmeticTarget::L)),2),
            0x06 => (Some(Instruction::RLC(ArithmeticTarget::AddressHL)),4),
            0x07 => (Some(Instruction::RLC(ArithmeticTarget::A)),2),
            0x08 => (Some(Instruction::RRC(ArithmeticTarget::B)),2),
            0x09 => (Some(Instruction::RRC(ArithmeticTarget::C)),2),
            0x0A => (Some(Instruction::RRC(ArithmeticTarget::D)),2),
            0x0B => (Some(Instruction::RRC(ArithmeticTarget::E)),2),
            0x0C => (Some(Instruction::RRC(ArithmeticTarget::H)),2),
            0x0D => (Some(Instruction::RRC(ArithmeticTarget::L)),2),
            0x0E => (Some(Instruction::RRC(ArithmeticTarget::AddressHL)),4),
            0x0F => (Some(Instruction::RRC(ArithmeticTarget::A)),2),

            0x10 => (Some(Instruction::RL(ArithmeticTarget::B)),2),
            0x11 => (Some(Instruction::RL(ArithmeticTarget::C)),2),
            0x12 => (Some(Instruction::RL(ArithmeticTarget::D)),2),
            0x13 => (Some(Instruction::RL(ArithmeticTarget::E)),2),
            0x14 => (Some(Instruction::RL(ArithmeticTarget::H)),2),
            0x15 => (Some(Instruction::RL(ArithmeticTarget::L)),2),
            0x16 => (Some(Instruction::RL(ArithmeticTarget::AddressHL)),4),
            0x17 => (Some(Instruction::RL(ArithmeticTarget::A)),2),
            0x18 => (Some(Instruction::RR(ArithmeticTarget::B)),2),
            0x19 => (Some(Instruction::RR(ArithmeticTarget::C)),2),
            0x1A => (Some(Instruction::RR(ArithmeticTarget::D)),2),
            0x1B => (Some(Instruction::RR(ArithmeticTarget::E)),2),
            0x1C => (Some(Instruction::RR(ArithmeticTarget::H)),2),
            0x1D => (Some(Instruction::RR(ArithmeticTarget::L)),2),
            0x1E => (Some(Instruction::RR(ArithmeticTarget::AddressHL)),4),
            0x1F => (Some(Instruction::RR(ArithmeticTarget::A)),2),

            0x20 => (Some(Instruction::SLA(ArithmeticTarget::B)),2),
            0x21 => (Some(Instruction::SLA(ArithmeticTarget::C)),2),
            0x22 => (Some(Instruction::SLA(ArithmeticTarget::D)),2),
            0x23 => (Some(Instruction::SLA(ArithmeticTarget::E)),2),
            0x24 => (Some(Instruction::SLA(ArithmeticTarget::H)),2),
            0x25 => (Some(Instruction::SLA(ArithmeticTarget::L)),2),
            0x26 => (Some(Instruction::SLA(ArithmeticTarget::AddressHL)),4),
            0x27 => (Some(Instruction::SLA(ArithmeticTarget::A)),2),
            0x28 => (Some(Instruction::SRA(ArithmeticTarget::B)),2),
            0x29 => (Some(Instruction::SRA(ArithmeticTarget::C)),2),
            0x2A => (Some(Instruction::SRA(ArithmeticTarget::D)),2),
            0x2B => (Some(Instruction::SRA(ArithmeticTarget::E)),2),
            0x2C => (Some(Instruction::SRA(ArithmeticTarget::H)),2),
            0x2D => (Some(Instruction::SRA(ArithmeticTarget::L)),2),
            0x2E => (Some(Instruction::SRA(ArithmeticTarget::AddressHL)),4),
            0x2F => (Some(Instruction::SRA(ArithmeticTarget::A)),2),

            0x30 => (Some(Instruction::SWAP(ArithmeticTarget::B)),2),
            0x31 => (Some(Instruction::SWAP(ArithmeticTarget::C)),2),
            0x32 => (Some(Instruction::SWAP(ArithmeticTarget::D)),2),
            0x33 => (Some(Instruction::SWAP(ArithmeticTarget::E)),2),
            0x34 => (Some(Instruction::SWAP(ArithmeticTarget::H)),2),
            0x35 => (Some(Instruction::SWAP(ArithmeticTarget::L)),2),
            0x36 => (Some(Instruction::SWAP(ArithmeticTarget::AddressHL)),4),
            0x37 => (Some(Instruction::SWAP(ArithmeticTarget::A)),2),
            0x38 => (Some(Instruction::SRL(ArithmeticTarget::B)),2),
            0x39 => (Some(Instruction::SRL(ArithmeticTarget::C)),2),
            0x3A => (Some(Instruction::SRL(ArithmeticTarget::D)),2),
            0x3B => (Some(Instruction::SRL(ArithmeticTarget::E)),2),
            0x3C => (Some(Instruction::SRL(ArithmeticTarget::H)),2),
            0x3D => (Some(Instruction::SRL(ArithmeticTarget::L)),2),
            0x3E => (Some(Instruction::SRL(ArithmeticTarget::AddressHL)),4),
            0x3F => (Some(Instruction::SRL(ArithmeticTarget::A)),2),

            0x40 => (Some(Instruction::BIT(ArithmeticTarget::B, 0)),2),
            0x41 => (Some(Instruction::BIT(ArithmeticTarget::C, 0)),2),
            0x42 => (Some(Instruction::BIT(ArithmeticTarget::D, 0)),2),
            0x43 => (Some(Instruction::BIT(ArithmeticTarget::E, 0)),2),
            0x44 => (Some(Instruction::BIT(ArithmeticTarget::H, 0)),2),
            0x45 => (Some(Instruction::BIT(ArithmeticTarget::L, 0)),2),
            0x46 => (Some(Instruction::BIT(ArithmeticTarget::AddressHL, 0)),4),
            0x47 => (Some(Instruction::BIT(ArithmeticTarget::A, 0)),2),
            0x48 => (Some(Instruction::BIT(ArithmeticTarget::B, 1)),2),
            0x49 => (Some(Instruction::BIT(ArithmeticTarget::C, 1)),2),
            0x4A => (Some(Instruction::BIT(ArithmeticTarget::D, 1)),2),
            0x4B => (Some(Instruction::BIT(ArithmeticTarget::E, 1)),2),
            0x4C => (Some(Instruction::BIT(ArithmeticTarget::H, 1)),2),
            0x4D => (Some(Instruction::BIT(ArithmeticTarget::L, 1)),2),
            0x4E => (Some(Instruction::BIT(ArithmeticTarget::AddressHL, 1)),4),
            0x4F => (Some(Instruction::BIT(ArithmeticTarget::A, 1)),2),

            // BIT instruction (bit 2)
            0x50 => (Some(Instruction::BIT(ArithmeticTarget::B, 2)),2),
            0x51 => (Some(Instruction::BIT(ArithmeticTarget::C, 2)),2),
            0x52 => (Some(Instruction::BIT(ArithmeticTarget::D, 2)),2),
            0x53 => (Some(Instruction::BIT(ArithmeticTarget::E, 2)),2),
            0x54 => (Some(Instruction::BIT(ArithmeticTarget::H, 2)),2),
            0x55 => (Some(Instruction::BIT(ArithmeticTarget::L, 2)),2),
            0x56 => (Some(Instruction::BIT(ArithmeticTarget::AddressHL, 2)),4),
            0x57 => (Some(Instruction::BIT(ArithmeticTarget::A, 2)), 2),

            // BIT instruction (bit 3)
            0x58 => (Some(Instruction::BIT(ArithmeticTarget::B, 3)),2),
            0x59 => (Some(Instruction::BIT(ArithmeticTarget::C, 3)),2),
            0x5A => (Some(Instruction::BIT(ArithmeticTarget::D, 3)),2),
            0x5B => (Some(Instruction::BIT(ArithmeticTarget::E, 3)),2),
            0x5C => (Some(Instruction::BIT(ArithmeticTarget::H, 3)),2),
            0x5D => (Some(Instruction::BIT(ArithmeticTarget::L, 3)),2),
            0x5E => (Some(Instruction::BIT(ArithmeticTarget::AddressHL, 3)),4),
            0x5F => (Some(Instruction::BIT(ArithmeticTarget::A, 3)),2),

            // BIT instruction (bit 4)
            0x60 => (Some(Instruction::BIT(ArithmeticTarget::B, 4)),2),
            0x61 => (Some(Instruction::BIT(ArithmeticTarget::C, 4)),2),
            0x62 => (Some(Instruction::BIT(ArithmeticTarget::D, 4)),2),
            0x63 => (Some(Instruction::BIT(ArithmeticTarget::E, 4)),2),
            0x64 => (Some(Instruction::BIT(ArithmeticTarget::H, 4)),2),
            0x65 => (Some(Instruction::BIT(ArithmeticTarget::L, 4)),2),
            0x66 => (Some(Instruction::BIT(ArithmeticTarget::AddressHL, 4)),4),
            0x67 => (Some(Instruction::BIT(ArithmeticTarget::A, 4)),2),

            // BIT instruction (bit 5)
            0x68 => (Some(Instruction::BIT(ArithmeticTarget::B, 5)),2),
            0x69 => (Some(Instruction::BIT(ArithmeticTarget::C, 5)),2),
            0x6A => (Some(Instruction::BIT(ArithmeticTarget::D, 5)),2),
            0x6B => (Some(Instruction::BIT(ArithmeticTarget::E, 5)),2),
            0x6C => (Some(Instruction::BIT(ArithmeticTarget::H, 5)),2),
            0x6D => (Some(Instruction::BIT(ArithmeticTarget::L, 5)),2),
            0x6E => (Some(Instruction::BIT(ArithmeticTarget::AddressHL, 5)),4),
            0x6F => (Some(Instruction::BIT(ArithmeticTarget::A, 5)),2),

            // BIT instruction (bit 6)
            0x70 => (Some(Instruction::BIT(ArithmeticTarget::B, 6)),2),
            0x71 => (Some(Instruction::BIT(ArithmeticTarget::C, 6)),2),
            0x72 => (Some(Instruction::BIT(ArithmeticTarget::D, 6)),2),
            0x73 => (Some(Instruction::BIT(ArithmeticTarget::E, 6)),2),
            0x74 => (Some(Instruction::BIT(ArithmeticTarget::H, 6)),2),
            0x75 => (Some(Instruction::BIT(ArithmeticTarget::L, 6)),2),
            0x76 => (Some(Instruction::BIT(ArithmeticTarget::AddressHL, 6)),4),
            0x77 => (Some(Instruction::BIT(ArithmeticTarget::A, 6)),2),

            // BIT instruction (bit 7)
            0x78 => (Some(Instruction::BIT(ArithmeticTarget::B, 7)),2),
            0x79 => (Some(Instruction::BIT(ArithmeticTarget::C, 7)),2),
            0x7A => (Some(Instruction::BIT(ArithmeticTarget::D, 7)),2),
            0x7B => (Some(Instruction::BIT(ArithmeticTarget::E, 7)),2),
            0x7C => (Some(Instruction::BIT(ArithmeticTarget::H, 7)),2),
            0x7D => (Some(Instruction::BIT(ArithmeticTarget::L, 7)),2),
            0x7E => (Some(Instruction::BIT(ArithmeticTarget::AddressHL, 7)),4),
            0x7F => (Some(Instruction::BIT(ArithmeticTarget::A, 7)),2),

            // RESET instruction (bit 0)
            0x80 => (Some(Instruction::RESET(ArithmeticTarget::B, 0)),2),
            0x81 => (Some(Instruction::RESET(ArithmeticTarget::C, 0)),2),
            0x82 => (Some(Instruction::RESET(ArithmeticTarget::D, 0)),2),
            0x83 => (Some(Instruction::RESET(ArithmeticTarget::E, 0)),2),
            0x84 => (Some(Instruction::RESET(ArithmeticTarget::H, 0)),2),
            0x85 => (Some(Instruction::RESET(ArithmeticTarget::L, 0)),2),
            0x86 => (Some(Instruction::RESET(ArithmeticTarget::AddressHL, 0)),4),
            0x87 => (Some(Instruction::RESET(ArithmeticTarget::A, 0)),2),

            // RESET instruction (bit 1)
            0x88 => (Some(Instruction::RESET(ArithmeticTarget::B, 1)),2),
            0x89 => (Some(Instruction::RESET(ArithmeticTarget::C, 1)),2),
            0x8A => (Some(Instruction::RESET(ArithmeticTarget::D, 1)),2),
            0x8B => (Some(Instruction::RESET(ArithmeticTarget::E, 1)),2),
            0x8C => (Some(Instruction::RESET(ArithmeticTarget::H, 1)),2),
            0x8D => (Some(Instruction::RESET(ArithmeticTarget::L, 1)),2),
            0x8E => (Some(Instruction::RESET(ArithmeticTarget::AddressHL, 1)),4),
            0x8F => (Some(Instruction::RESET(ArithmeticTarget::A, 1)),2),

            // RESET instruction (bit 2)
            0x90 => (Some(Instruction::RESET(ArithmeticTarget::B, 2)),2),
            0x91 => (Some(Instruction::RESET(ArithmeticTarget::C, 2)),2),
            0x92 => (Some(Instruction::RESET(ArithmeticTarget::D, 2)),2),
            0x93 => (Some(Instruction::RESET(ArithmeticTarget::E, 2)),2),
            0x94 => (Some(Instruction::RESET(ArithmeticTarget::H, 2)),2),
            0x95 => (Some(Instruction::RESET(ArithmeticTarget::L, 2)),2),
            0x96 => (Some(Instruction::RESET(ArithmeticTarget::AddressHL, 2)),4),
            0x97 => (Some(Instruction::RESET(ArithmeticTarget::A, 2)),2),

            // RESET instruction (bit 3)
            0x98 => (Some(Instruction::RESET(ArithmeticTarget::B, 3)),2),
            0x99 => (Some(Instruction::RESET(ArithmeticTarget::C, 3)),2),
            0x9A => (Some(Instruction::RESET(ArithmeticTarget::D, 3)),2),
            0x9B => (Some(Instruction::RESET(ArithmeticTarget::E, 3)),2),
            0x9C => (Some(Instruction::RESET(ArithmeticTarget::H, 3)),2),
            0x9D => (Some(Instruction::RESET(ArithmeticTarget::L, 3)),2),
            0x9E => (Some(Instruction::RESET(ArithmeticTarget::AddressHL, 3)),4),
            0x9F => (Some(Instruction::RESET(ArithmeticTarget::A, 3)),2),

            // RESET instruction (bit 4)
            0xA0 => (Some(Instruction::RESET(ArithmeticTarget::B, 4)),2),
            0xA1 => (Some(Instruction::RESET(ArithmeticTarget::C, 4)),2),
            0xA2 => (Some(Instruction::RESET(ArithmeticTarget::D, 4)),2),
            0xA3 => (Some(Instruction::RESET(ArithmeticTarget::E, 4)),2),
            0xA4 => (Some(Instruction::RESET(ArithmeticTarget::H, 4)),2),
            0xA5 => (Some(Instruction::RESET(ArithmeticTarget::L, 4)),2),
            0xA6 => (Some(Instruction::RESET(ArithmeticTarget::AddressHL, 4)),2),
            0xA7 => (Some(Instruction::RESET(ArithmeticTarget::A, 4)),2),

            // RESET instruction (bit 5)
            0xA8 => (Some(Instruction::RESET(ArithmeticTarget::B, 5)),2),
            0xA9 => (Some(Instruction::RESET(ArithmeticTarget::C, 5)),2),
            0xAA => (Some(Instruction::RESET(ArithmeticTarget::D, 5)),2),
            0xAB => (Some(Instruction::RESET(ArithmeticTarget::E, 5)),2),
            0xAC => (Some(Instruction::RESET(ArithmeticTarget::H, 5)),2),
            0xAD => (Some(Instruction::RESET(ArithmeticTarget::L, 5)),2),
            0xAE => (Some(Instruction::RESET(ArithmeticTarget::AddressHL, 5)),4),
            0xAF => (Some(Instruction::RESET(ArithmeticTarget::A, 5)),2),

            // RESET instruction (bit 6)
            0xB0 => (Some(Instruction::RESET(ArithmeticTarget::B, 6)),2),
            0xB1 => (Some(Instruction::RESET(ArithmeticTarget::C, 6)),2),
            0xB2 => (Some(Instruction::RESET(ArithmeticTarget::D, 6)),2),
            0xB3 => (Some(Instruction::RESET(ArithmeticTarget::E, 6)),2),
            0xB4 => (Some(Instruction::RESET(ArithmeticTarget::H, 6)),2),
            0xB5 => (Some(Instruction::RESET(ArithmeticTarget::L, 6)),2),
            0xB6 => (Some(Instruction::RESET(ArithmeticTarget::AddressHL, 6)),4),
            0xB7 => (Some(Instruction::RESET(ArithmeticTarget::A, 6)),2),

            // RESET instruction (bit 7)
            0xB8 => (Some(Instruction::RESET(ArithmeticTarget::B, 7)),2),
            0xB9 => (Some(Instruction::RESET(ArithmeticTarget::C, 7)),2),
            0xBA => (Some(Instruction::RESET(ArithmeticTarget::D, 7)),2),
            0xBB => (Some(Instruction::RESET(ArithmeticTarget::E, 7)),2),
            0xBC => (Some(Instruction::RESET(ArithmeticTarget::H, 7)),2),
            0xBD => (Some(Instruction::RESET(ArithmeticTarget::L, 7)),2),
            0xBE => (Some(Instruction::RESET(ArithmeticTarget::AddressHL, 7)),4),
            0xBF => (Some(Instruction::RESET(ArithmeticTarget::A, 7)),2),

            // SET instruction (bit 0)
            0xC0 => (Some(Instruction::SET(ArithmeticTarget::B, 0)),2),
            0xC1 => (Some(Instruction::SET(ArithmeticTarget::C, 0)),2),
            0xC2 => (Some(Instruction::SET(ArithmeticTarget::D, 0)),2),
            0xC3 => (Some(Instruction::SET(ArithmeticTarget::E, 0)),2),
            0xC4 => (Some(Instruction::SET(ArithmeticTarget::H, 0)),2),
            0xC5 => (Some(Instruction::SET(ArithmeticTarget::L, 0)),2),
            0xC6 => (Some(Instruction::SET(ArithmeticTarget::AddressHL, 0)),4),
            0xC7 => (Some(Instruction::SET(ArithmeticTarget::A, 0)),2),

            // SET instruction (bit 1)
            0xC8 => (Some(Instruction::SET(ArithmeticTarget::B, 1)),2),
            0xC9 => (Some(Instruction::SET(ArithmeticTarget::C, 1)),2),
            0xCA => (Some(Instruction::SET(ArithmeticTarget::D, 1)),2),
            0xCB => (Some(Instruction::SET(ArithmeticTarget::E, 1)),2),
            0xCC => (Some(Instruction::SET(ArithmeticTarget::H, 1)),2),
            0xCD => (Some(Instruction::SET(ArithmeticTarget::L, 1)),2),
            0xCE => (Some(Instruction::SET(ArithmeticTarget::AddressHL, 1)),4),
            0xCF => (Some(Instruction::SET(ArithmeticTarget::A, 1)),2),

            // SET instruction (bit 2)
            0xD0 => (Some(Instruction::SET(ArithmeticTarget::B, 2)),2),
            0xD1 => (Some(Instruction::SET(ArithmeticTarget::C, 2)),2),
            0xD2 => (Some(Instruction::SET(ArithmeticTarget::D, 2)),2),
            0xD3 => (Some(Instruction::SET(ArithmeticTarget::E, 2)),2),
            0xD4 => (Some(Instruction::SET(ArithmeticTarget::H, 2)),2),
            0xD5 => (Some(Instruction::SET(ArithmeticTarget::L, 2)),2),
            0xD6 => (Some(Instruction::SET(ArithmeticTarget::AddressHL, 2)),4),
            0xD7 => (Some(Instruction::SET(ArithmeticTarget::A, 2)),2),

            // SET instruction (bit 3)
            0xD8 => (Some(Instruction::SET(ArithmeticTarget::B, 3)),2),
            0xD9 => (Some(Instruction::SET(ArithmeticTarget::C, 3)),2),
            0xDA => (Some(Instruction::SET(ArithmeticTarget::D, 3)),2),
            0xDB => (Some(Instruction::SET(ArithmeticTarget::E, 3)),2),
            0xDC => (Some(Instruction::SET(ArithmeticTarget::H, 3)),2),
            0xDD => (Some(Instruction::SET(ArithmeticTarget::L, 3)),2),
            0xDE => (Some(Instruction::SET(ArithmeticTarget::AddressHL, 3)),4),
            0xDF => (Some(Instruction::SET(ArithmeticTarget::A, 3)),2),

            // SET instruction (bit 4)
            0xE0 => (Some(Instruction::SET(ArithmeticTarget::B, 4)),2),
            0xE1 => (Some(Instruction::SET(ArithmeticTarget::C, 4)),2),
            0xE2 => (Some(Instruction::SET(ArithmeticTarget::D, 4)),2),
            0xE3 => (Some(Instruction::SET(ArithmeticTarget::E, 4)),2),
            0xE4 => (Some(Instruction::SET(ArithmeticTarget::H, 4)),2),
            0xE5 => (Some(Instruction::SET(ArithmeticTarget::L, 4)),2),
            0xE6 => (Some(Instruction::SET(ArithmeticTarget::AddressHL, 4)),4),
            0xE7 => (Some(Instruction::SET(ArithmeticTarget::A, 4)),2),

            // SET instruction (bit 5)
            0xE8 => (Some(Instruction::SET(ArithmeticTarget::B, 5)),2),
            0xE9 => (Some(Instruction::SET(ArithmeticTarget::C, 5)),2),
            0xEA => (Some(Instruction::SET(ArithmeticTarget::D, 5)),2),
            0xEB => (Some(Instruction::SET(ArithmeticTarget::E, 5)),2),
            0xEC => (Some(Instruction::SET(ArithmeticTarget::H, 5)),2),
            0xED => (Some(Instruction::SET(ArithmeticTarget::L, 5)),2),
            0xEE => (Some(Instruction::SET(ArithmeticTarget::AddressHL, 5)),4),
            0xEF => (Some(Instruction::SET(ArithmeticTarget::A, 5)),2),

            // SET instruction (bit 6)
            0xF0 => (Some(Instruction::SET(ArithmeticTarget::B, 6)),2),
            0xF1 => (Some(Instruction::SET(ArithmeticTarget::C, 6)),2),
            0xF2 => (Some(Instruction::SET(ArithmeticTarget::D, 6)),2),
            0xF3 => (Some(Instruction::SET(ArithmeticTarget::E, 6)),2),
            0xF4 => (Some(Instruction::SET(ArithmeticTarget::H, 6)),2),
            0xF5 => (Some(Instruction::SET(ArithmeticTarget::L, 6)),2),
            0xF6 => (Some(Instruction::SET(ArithmeticTarget::AddressHL, 6)),4),
            0xF7 => (Some(Instruction::SET(ArithmeticTarget::A, 6)),2),

            // SET instruction (bit 7)
            0xF8 => (Some(Instruction::SET(ArithmeticTarget::B, 7)),2),
            0xF9 => (Some(Instruction::SET(ArithmeticTarget::C, 7)),2),
            0xFA => (Some(Instruction::SET(ArithmeticTarget::D, 7)),2),
            0xFB => (Some(Instruction::SET(ArithmeticTarget::E, 7)),2),
            0xFC => (Some(Instruction::SET(ArithmeticTarget::H, 7)),2),
            0xFD => (Some(Instruction::SET(ArithmeticTarget::L, 7)),2),
            0xFE => (Some(Instruction::SET(ArithmeticTarget::AddressHL, 7)),4),
            0xFF => (Some(Instruction::SET(ArithmeticTarget::A, 7)),2),
        }
    }

    pub fn from_byte_not_prefixed(byte: u8) -> (Option<Instruction>,u8) {
        match byte {
            0x00 => (Some(Instruction::NOP),1),
            0x01 => (Some(Instruction::LD(LoadType::Word(LoadWordTarget::BC, LoadWordSource::D16))),3),
            0x02 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::A))),2),
            0x03 => (Some(Instruction::INC(ArithmeticTarget::BC)),2),
            0x04 => (Some(Instruction::INC(ArithmeticTarget::B)),1),
            0x05 => (Some(Instruction::DEC(ArithmeticTarget::B)),1),
            0x06 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),2),
            0x07 => (Some(Instruction::RLCA),1),
            0x08 => (Some(Instruction::LD(LoadType::Word(LoadWordTarget::Address16, LoadWordSource::SP))),5),
            0x09 => (Some(Instruction::ADDHL(ArithmeticTarget::BC)),2),
            0x0A => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressBC))),2),
            0x0B => (Some(Instruction::DEC(ArithmeticTarget::BC)),2),
            0x0C => (Some(Instruction::INC(ArithmeticTarget::C)),1),
            0x0D => (Some(Instruction::DEC(ArithmeticTarget::C)),1),
            0x0E => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),2),
            0x0F => (Some(Instruction::RRCA),1),

            0x10 => (Some(Instruction::STOP),1),
            0x11 => (Some(Instruction::LD(LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16))),3),
            0x12 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressDE, LoadByteSource::A))),2),
            0x13 => (Some(Instruction::INC(ArithmeticTarget::DE)),2),
            0x14 => (Some(Instruction::INC(ArithmeticTarget::D)),1),
            0x15 => (Some(Instruction::DEC(ArithmeticTarget::D)),1),
            0x16 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),2),
            0x17 => (Some(Instruction::RLA),1),
            0x18 => (Some(Instruction::JR(JumpTest::Always)),2), // Remplacez "offset" par la valeur appropriée.
            0x19 => (Some(Instruction::ADDHL(ArithmeticTarget::DE)),2),
            0x1A => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressDE))),2),
            0x1B => (Some(Instruction::DEC(ArithmeticTarget::DE)),2),
            0x1C => (Some(Instruction::INC(ArithmeticTarget::E)),1),
            0x1D => (Some(Instruction::DEC(ArithmeticTarget::E)),1),
            0x1E => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),2),
            0x1F => (Some(Instruction::RRA),1),
                                                                                                                                        
            0x20 => (Some(Instruction::JR(JumpTest::NotZero)),2), // Remplacez "offset" par la valeur appropriée.
            0x21 => (Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::D16))),3),
            0x22 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHLP, LoadByteSource::A))),2),
            0x23 => (Some(Instruction::INC(ArithmeticTarget::HL)),2),
            0x24 => (Some(Instruction::INC(ArithmeticTarget::H)),1),
            0x25 => (Some(Instruction::DEC(ArithmeticTarget::H)),1),
            0x26 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),2),
            0x27 => (Some(Instruction::DAA),1),
            0x28 => (Some(Instruction::JR(JumpTest::Zero)),2), // Remplacez "offset" par la valeur appropriée.
            0x29 => (Some(Instruction::ADDHL(ArithmeticTarget::HL)),2),
            0x2A => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressHLP))),2),
            0x2B => (Some(Instruction::DEC(ArithmeticTarget::HL)),2),
            0x2C => (Some(Instruction::INC(ArithmeticTarget::L)),1),
            0x2D => (Some(Instruction::DEC(ArithmeticTarget::L)),1),
            0x2E => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),2),
            0x2F => (Some(Instruction::CPL),1),
                                                                                                                                                                                                        
            0x30 => (Some(Instruction::JR(JumpTest::NotCarry)),2), // Remplacez "offset" par la valeur appropriée.
            0x31 => (Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::D16))),3),
            0x32 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHLM, LoadByteSource::A))),2),
            0x33 => (Some(Instruction::INC(ArithmeticTarget::SP)),2),
            0x34 => (Some(Instruction::INC(ArithmeticTarget::AddressHL)),3),
            0x35 => (Some(Instruction::DEC(ArithmeticTarget::AddressHL)),3),
            0x36 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::D8))),3),
            0x37 => (Some(Instruction::SCF),1),
            0x38 => (Some(Instruction::JR(JumpTest::Carry)),2), // Remplacez "offset" par la valeur appropriée.
            0x39 => (Some(Instruction::ADDHL(ArithmeticTarget::SP)),2),
            0x3A => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressHLM))),2),
            0x3B => (Some(Instruction::DEC(ArithmeticTarget::SP)),2),
            0x3C => (Some(Instruction::INC(ArithmeticTarget::A)),1),
            0x3D => (Some(Instruction::DEC(ArithmeticTarget::A)),1),
            0x3E => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),2),
            0x3F => (Some(Instruction::CCF),1),
                                                                                                                                                                                                                                                                        
            0x40 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),1),
            0x41 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),1),
            0x42 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),1),
            0x43 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),1),
            0x44 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),1),
            0x45 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),1),
            0x46 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::AddressHL))),2),
            0x47 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),1),
            0x48 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),1),
            0x49 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),1),
            0x4A => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),1),
            0x4B => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),1),
            0x4C => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))),1),
            0x4D => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),1),
            0x4E => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::AddressHL))),2),
            0x4F => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),1),
                                                                                                                                                                                                                                                                                                                                        
            0x50 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),1),
            0x51 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),1),
            0x52 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),1),
            0x53 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),1),
            0x54 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),1),
            0x55 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),1),
            0x56 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::AddressHL))),2),
            0x57 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),1),
            0x58 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),1),
            0x59 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),1),
            0x5A => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),1),
            0x5B => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),1),
            0x5C => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),1),
            0x5D => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),1),
            0x5E => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::AddressHL))),2),
            0x5F => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),1),
                                                                                                                                                                                                                                                                                                                                                                                                        
            0x60 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),1),
            0x61 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),1),
            0x62 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))),1),
            0x63 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))),1),
            0x64 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),1),
            0x65 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),1),
            0x66 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::AddressHL))),2),
            0x67 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),1),
            0x68 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),1),
            0x69 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),1),
            0x6A => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),1),
            0x6B => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),1),
            0x6C => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),1),
            0x6D => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),1),
            0x6E => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::AddressHL))),2),
            0x6F => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0x70 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::B))),2),
            0x71 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::C))),2),
            0x72 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::D))),2),
            0x73 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::E))),2),
            0x74 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::H))),2),
            0x75 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::L))),2),
            0x76 => (Some(Instruction::HALT),1),
            0x77 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::A))),2),
            0x78 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),1),
            0x79 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),1),
            0x7A => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),1),
            0x7B => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),1),
            0x7C => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),1),
            0x7D => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),1),
            0x7E => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressHL))),2),
            0x7F => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0x80 => (Some(Instruction::ADD(ArithmeticTarget::B)),1),
            0x81 => (Some(Instruction::ADD(ArithmeticTarget::C)),1),
            0x82 => (Some(Instruction::ADD(ArithmeticTarget::D)),1),
            0x83 => (Some(Instruction::ADD(ArithmeticTarget::E)),1),
            0x84 => (Some(Instruction::ADD(ArithmeticTarget::H)),1),
            0x85 => (Some(Instruction::ADD(ArithmeticTarget::L)),1),
            0x86 => (Some(Instruction::ADD(ArithmeticTarget::AddressHL)),2),
            0x87 => (Some(Instruction::ADD(ArithmeticTarget::A)),1),
            0x88 => (Some(Instruction::ADC(ArithmeticTarget::B)),1),
            0x89 => (Some(Instruction::ADC(ArithmeticTarget::C)),1),
            0x8A => (Some(Instruction::ADC(ArithmeticTarget::D)),1),
            0x8B => (Some(Instruction::ADC(ArithmeticTarget::E)),1),
            0x8C => (Some(Instruction::ADC(ArithmeticTarget::H)),1),
            0x8D => (Some(Instruction::ADC(ArithmeticTarget::L)),1),
            0x8E => (Some(Instruction::ADC(ArithmeticTarget::AddressHL)),2),
            0x8F => (Some(Instruction::ADC(ArithmeticTarget::A)),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0x90 => (Some(Instruction::SUB(ArithmeticTarget::B)),1),
            0x91 => (Some(Instruction::SUB(ArithmeticTarget::C)),1),
            0x92 => (Some(Instruction::SUB(ArithmeticTarget::D)),1),
            0x93 => (Some(Instruction::SUB(ArithmeticTarget::E)),1),
            0x94 => (Some(Instruction::SUB(ArithmeticTarget::H)),1),
            0x95 => (Some(Instruction::SUB(ArithmeticTarget::L)),1),
            0x96 => (Some(Instruction::SUB(ArithmeticTarget::AddressHL)),2),
            0x97 => (Some(Instruction::SUB(ArithmeticTarget::A)),1),
            0x98 => (Some(Instruction::SBC(ArithmeticTarget::B)),1),
            0x99 => (Some(Instruction::SBC(ArithmeticTarget::C)),1),
            0x9A => (Some(Instruction::SBC(ArithmeticTarget::D)),1),
            0x9B => (Some(Instruction::SBC(ArithmeticTarget::E)),1),
            0x9C => (Some(Instruction::SBC(ArithmeticTarget::H)),1),
            0x9D => (Some(Instruction::SBC(ArithmeticTarget::L)),1),
            0x9E => (Some(Instruction::SBC(ArithmeticTarget::AddressHL)),2),
            0x9F => (Some(Instruction::SBC(ArithmeticTarget::A)),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xA0 => (Some(Instruction::AND(ArithmeticTarget::B)),1),
            0xA1 => (Some(Instruction::AND(ArithmeticTarget::C)),1),
            0xA2 => (Some(Instruction::AND(ArithmeticTarget::D)),1),
            0xA3 => (Some(Instruction::AND(ArithmeticTarget::E)),1),
            0xA4 => (Some(Instruction::AND(ArithmeticTarget::H)),1),
            0xA5 => (Some(Instruction::AND(ArithmeticTarget::L)),1),
            0xA6 => (Some(Instruction::AND(ArithmeticTarget::AddressHL)),2),
            0xA7 => (Some(Instruction::AND(ArithmeticTarget::A)),1),
            0xA8 => (Some(Instruction::XOR(ArithmeticTarget::B)),1),
            0xA9 => (Some(Instruction::XOR(ArithmeticTarget::C)),1),
            0xAA => (Some(Instruction::XOR(ArithmeticTarget::D)),1),
            0xAB => (Some(Instruction::XOR(ArithmeticTarget::E)),1),
            0xAC => (Some(Instruction::XOR(ArithmeticTarget::H)),1),
            0xAD => (Some(Instruction::XOR(ArithmeticTarget::L)),1),
            0xAE => (Some(Instruction::XOR(ArithmeticTarget::AddressHL)),2),
            0xAF => (Some(Instruction::XOR(ArithmeticTarget::A)),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xB0 => (Some(Instruction::OR(ArithmeticTarget::B)),1),
            0xB1 => (Some(Instruction::OR(ArithmeticTarget::C)),1),
            0xB2 => (Some(Instruction::OR(ArithmeticTarget::D)),1),
            0xB3 => (Some(Instruction::OR(ArithmeticTarget::E)),1),
            0xB4 => (Some(Instruction::OR(ArithmeticTarget::H)),1),
            0xB5 => (Some(Instruction::OR(ArithmeticTarget::L)),1),
            0xB6 => (Some(Instruction::OR(ArithmeticTarget::AddressHL)),2),
            0xB7 => (Some(Instruction::OR(ArithmeticTarget::A)),1),
            0xB8 => (Some(Instruction::CP(ArithmeticTarget::B)),1),
            0xB9 => (Some(Instruction::CP(ArithmeticTarget::C)),1),
            0xBA => (Some(Instruction::CP(ArithmeticTarget::D)),1),
            0xBB => (Some(Instruction::CP(ArithmeticTarget::E)),1),
            0xBC => (Some(Instruction::CP(ArithmeticTarget::H)),1),
            0xBD => (Some(Instruction::CP(ArithmeticTarget::L)),1),
            0xBE => (Some(Instruction::CP(ArithmeticTarget::AddressHL)),2),
            0xBF => (Some(Instruction::CP(ArithmeticTarget::A)),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xC0 => (Some(Instruction::RET(JumpTest::NotZero)),2),
            0xC1 => (Some(Instruction::POP(StackTarget::BC)),3),
            0xC2 => (Some(Instruction::JP(JumpTest::NotZero, JumpCondition::Address16)),3),
            0xC3 => (Some(Instruction::JP(JumpTest::Always, JumpCondition::Address16)),3),
            0xC4 => (Some(Instruction::CALL(JumpTest::NotZero)),3),
            0xC5 => (Some(Instruction::PUSH(StackTarget::BC)),4),
            0xC6 => (Some(Instruction::ADD(ArithmeticTarget::D8)),2),
            0xC7 => (Some(Instruction::RST(RstTarget::Rst00H)),4),
            0xC8 => (Some(Instruction::RET(JumpTest::Zero)),2),
            0xC9 => (Some(Instruction::RET(JumpTest::Always)),1),
            0xCA => (Some(Instruction::JP(JumpTest::Zero, JumpCondition::Address16)),3),
            0xCB => (Some(Instruction::PrefixCB),1),
            0xCC => (Some(Instruction::CALL(JumpTest::Zero)),3),
            0xCD => (Some(Instruction::CALL(JumpTest::Always)),3),
            0xCE => (Some(Instruction::ADC(ArithmeticTarget::D8)),2),
            0xCF => (Some(Instruction::RST(RstTarget::Rst08H)),4),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xD0 => (Some(Instruction::RET(JumpTest::NotCarry)),2),
            0xD1 => (Some(Instruction::POP(StackTarget::DE)),3),
            0xD2 => (Some(Instruction::JP(JumpTest::NotCarry, JumpCondition::Address16)),3),
            0xD4 => (Some(Instruction::CALL(JumpTest::NotCarry)),3),
            0xD5 => (Some(Instruction::PUSH(StackTarget::DE)),4),
            0xD6 => (Some(Instruction::SUB(ArithmeticTarget::D8)),2),
            0xD7 => (Some(Instruction::RST(RstTarget::Rst10H)),4),
            0xD8 => (Some(Instruction::RET(JumpTest::Carry)),2),
            0xD9 => (Some(Instruction::RETI),4),
            0xDA => (Some(Instruction::JP(JumpTest::Carry, JumpCondition::Address16)),3),
            0xDC => (Some(Instruction::CALL(JumpTest::Carry)),3),
            0xDE => (Some(Instruction::SBC(ArithmeticTarget::D8)),2),
            0xDF => (Some(Instruction::RST(RstTarget::Rst18H)),4),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xE0 => (Some(Instruction::LDH(LoadType::Byte(LoadByteTarget::Address8, LoadByteSource::A))),3),
            0xE1 => (Some(Instruction::POP(StackTarget::HL)),3),
            0xE2 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressC, LoadByteSource::A))),2),
            0xE5 => (Some(Instruction::PUSH(StackTarget::HL)),4),
            0xE6 => (Some(Instruction::AND(ArithmeticTarget::D8)),2),
            0xE7 => (Some(Instruction::RST(RstTarget::Rst20H)),4),
            0xE8 => (Some(Instruction::ADDSP()),4),
            0xE9 => (Some(Instruction::JP(JumpTest::Always, JumpCondition::AddressHL)),1),
            0xEA => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::Address16, LoadByteSource::A))),4),
            0xEE => (Some(Instruction::XOR(ArithmeticTarget::D8)),2),
            0xEF => (Some(Instruction::RST(RstTarget::Rst28H)),4),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xF0 => (Some(Instruction::LDH(LoadType::Byte(LoadByteTarget::A, LoadByteSource::Address8))),3),
            0xF1 => (Some(Instruction::POP(StackTarget::AF)),3),
            0xF2 => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressC))),2),
            0xF3 => (Some(Instruction::DI),1),
            0xF5 => (Some(Instruction::PUSH(StackTarget::AF)),4),
            0xF6 => (Some(Instruction::OR(ArithmeticTarget::D8)),2),
            0xF7 => (Some(Instruction::RST(RstTarget::Rst30H)),4),
            0xF8 => (Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::SPR8))),3),
            0xF9 => (Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::HL))),2),
            0xFA => (Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::Address16))),4),
            0xFB => (Some(Instruction::EI),1),
            0xFE => (Some(Instruction::CP(ArithmeticTarget::D8)),2),
            0xFF => (Some(Instruction::RST(RstTarget::Rst38H)),4),
            _ => {panic!("unknown prefixed instruction")}
        }
    }
}
