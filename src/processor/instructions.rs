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
    A, B, C, D, E, H, L, AddressHL, AddressBC, AddressDE, AddressHLP, AddressHLM, AddressC, Address16, Address8
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
    Nop,
    Add(ArithmeticTarget),
    Addhl(ArithmeticTarget),
    Addsp(),
    Adc(ArithmeticTarget),
    Sub(ArithmeticTarget),
    Sbc(ArithmeticTarget),
    And(ArithmeticTarget),
    Or(ArithmeticTarget),
    Xor(ArithmeticTarget),
    Cp(ArithmeticTarget),
    Inc(ArithmeticTarget),
    Dec(ArithmeticTarget),
    Ccf,
    Scf,
    Daa,
    Cpl,
    Rra,
    Rla,
    Rrca,
    Rlca,
    Bit(ArithmeticTarget, u8), // ArithmeticTarget represents the register to test and u8 represents the bit number (0-7).
    Reset(ArithmeticTarget, u8), // ArithmeticTarget represents the register to reset and u8 represents the bit number (0-7).
    Set(ArithmeticTarget, u8), // ArithmeticTarget represents the register to set and u8 represents the bit number (0-7).
    Srl(ArithmeticTarget),
    Rr(ArithmeticTarget),
    Rl(ArithmeticTarget),
    Rrc(ArithmeticTarget),
    Rlc(ArithmeticTarget),
    Sra(ArithmeticTarget),
    Sla(ArithmeticTarget),
    Swap(ArithmeticTarget),
    Ld(LoadType),
    Ldh(LoadType),
    Jp(JumpTest, JumpCondition),
    Push(StackTarget),
    Pop(StackTarget),
    Call(JumpTest),
    Ret(JumpTest),
    Jr(JumpTest),
    Reti,
    Stop,
    Halt,
    Ei,
    Di,
    Rst(RstTarget),
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
            0x00 => (Some(Instruction::Rlc(ArithmeticTarget::B)),2),
            0x01 => (Some(Instruction::Rlc(ArithmeticTarget::C)),2),
            0x02 => (Some(Instruction::Rlc(ArithmeticTarget::D)),2),
            0x03 => (Some(Instruction::Rlc(ArithmeticTarget::E)),2),
            0x04 => (Some(Instruction::Rlc(ArithmeticTarget::H)),2),
            0x05 => (Some(Instruction::Rlc(ArithmeticTarget::L)),2),
            0x06 => (Some(Instruction::Rlc(ArithmeticTarget::AddressHL)),4),
            0x07 => (Some(Instruction::Rlc(ArithmeticTarget::A)),2),
            0x08 => (Some(Instruction::Rrc(ArithmeticTarget::B)),2),
            0x09 => (Some(Instruction::Rrc(ArithmeticTarget::C)),2),
            0x0A => (Some(Instruction::Rrc(ArithmeticTarget::D)),2),
            0x0B => (Some(Instruction::Rrc(ArithmeticTarget::E)),2),
            0x0C => (Some(Instruction::Rrc(ArithmeticTarget::H)),2),
            0x0D => (Some(Instruction::Rrc(ArithmeticTarget::L)),2),
            0x0E => (Some(Instruction::Rrc(ArithmeticTarget::AddressHL)),4),
            0x0F => (Some(Instruction::Rrc(ArithmeticTarget::A)),2),

            0x10 => (Some(Instruction::Rl(ArithmeticTarget::B)),2),
            0x11 => (Some(Instruction::Rl(ArithmeticTarget::C)),2),
            0x12 => (Some(Instruction::Rl(ArithmeticTarget::D)),2),
            0x13 => (Some(Instruction::Rl(ArithmeticTarget::E)),2),
            0x14 => (Some(Instruction::Rl(ArithmeticTarget::H)),2),
            0x15 => (Some(Instruction::Rl(ArithmeticTarget::L)),2),
            0x16 => (Some(Instruction::Rl(ArithmeticTarget::AddressHL)),4),
            0x17 => (Some(Instruction::Rl(ArithmeticTarget::A)),2),
            0x18 => (Some(Instruction::Rr(ArithmeticTarget::B)),2),
            0x19 => (Some(Instruction::Rr(ArithmeticTarget::C)),2),
            0x1A => (Some(Instruction::Rr(ArithmeticTarget::D)),2),
            0x1B => (Some(Instruction::Rr(ArithmeticTarget::E)),2),
            0x1C => (Some(Instruction::Rr(ArithmeticTarget::H)),2),
            0x1D => (Some(Instruction::Rr(ArithmeticTarget::L)),2),
            0x1E => (Some(Instruction::Rr(ArithmeticTarget::AddressHL)),4),
            0x1F => (Some(Instruction::Rr(ArithmeticTarget::A)),2),

            0x20 => (Some(Instruction::Sla(ArithmeticTarget::B)),2),
            0x21 => (Some(Instruction::Sla(ArithmeticTarget::C)),2),
            0x22 => (Some(Instruction::Sla(ArithmeticTarget::D)),2),
            0x23 => (Some(Instruction::Sla(ArithmeticTarget::E)),2),
            0x24 => (Some(Instruction::Sla(ArithmeticTarget::H)),2),
            0x25 => (Some(Instruction::Sla(ArithmeticTarget::L)),2),
            0x26 => (Some(Instruction::Sla(ArithmeticTarget::AddressHL)),4),
            0x27 => (Some(Instruction::Sla(ArithmeticTarget::A)),2),
            0x28 => (Some(Instruction::Sra(ArithmeticTarget::B)),2),
            0x29 => (Some(Instruction::Sra(ArithmeticTarget::C)),2),
            0x2A => (Some(Instruction::Sra(ArithmeticTarget::D)),2),
            0x2B => (Some(Instruction::Sra(ArithmeticTarget::E)),2),
            0x2C => (Some(Instruction::Sra(ArithmeticTarget::H)),2),
            0x2D => (Some(Instruction::Sra(ArithmeticTarget::L)),2),
            0x2E => (Some(Instruction::Sra(ArithmeticTarget::AddressHL)),4),
            0x2F => (Some(Instruction::Sra(ArithmeticTarget::A)),2),

            0x30 => (Some(Instruction::Swap(ArithmeticTarget::B)),2),
            0x31 => (Some(Instruction::Swap(ArithmeticTarget::C)),2),
            0x32 => (Some(Instruction::Swap(ArithmeticTarget::D)),2),
            0x33 => (Some(Instruction::Swap(ArithmeticTarget::E)),2),
            0x34 => (Some(Instruction::Swap(ArithmeticTarget::H)),2),
            0x35 => (Some(Instruction::Swap(ArithmeticTarget::L)),2),
            0x36 => (Some(Instruction::Swap(ArithmeticTarget::AddressHL)),4),
            0x37 => (Some(Instruction::Swap(ArithmeticTarget::A)),2),
            0x38 => (Some(Instruction::Srl(ArithmeticTarget::B)),2),
            0x39 => (Some(Instruction::Srl(ArithmeticTarget::C)),2),
            0x3A => (Some(Instruction::Srl(ArithmeticTarget::D)),2),
            0x3B => (Some(Instruction::Srl(ArithmeticTarget::E)),2),
            0x3C => (Some(Instruction::Srl(ArithmeticTarget::H)),2),
            0x3D => (Some(Instruction::Srl(ArithmeticTarget::L)),2),
            0x3E => (Some(Instruction::Srl(ArithmeticTarget::AddressHL)),4),
            0x3F => (Some(Instruction::Srl(ArithmeticTarget::A)),2),

            0x40 => (Some(Instruction::Bit(ArithmeticTarget::B, 0)),2),
            0x41 => (Some(Instruction::Bit(ArithmeticTarget::C, 0)),2),
            0x42 => (Some(Instruction::Bit(ArithmeticTarget::D, 0)),2),
            0x43 => (Some(Instruction::Bit(ArithmeticTarget::E, 0)),2),
            0x44 => (Some(Instruction::Bit(ArithmeticTarget::H, 0)),2),
            0x45 => (Some(Instruction::Bit(ArithmeticTarget::L, 0)),2),
            0x46 => (Some(Instruction::Bit(ArithmeticTarget::AddressHL, 0)),4),
            0x47 => (Some(Instruction::Bit(ArithmeticTarget::A, 0)),2),
            0x48 => (Some(Instruction::Bit(ArithmeticTarget::B, 1)),2),
            0x49 => (Some(Instruction::Bit(ArithmeticTarget::C, 1)),2),
            0x4A => (Some(Instruction::Bit(ArithmeticTarget::D, 1)),2),
            0x4B => (Some(Instruction::Bit(ArithmeticTarget::E, 1)),2),
            0x4C => (Some(Instruction::Bit(ArithmeticTarget::H, 1)),2),
            0x4D => (Some(Instruction::Bit(ArithmeticTarget::L, 1)),2),
            0x4E => (Some(Instruction::Bit(ArithmeticTarget::AddressHL, 1)),4),
            0x4F => (Some(Instruction::Bit(ArithmeticTarget::A, 1)),2),

            // BIT instruction (bit 2)
            0x50 => (Some(Instruction::Bit(ArithmeticTarget::B, 2)),2),
            0x51 => (Some(Instruction::Bit(ArithmeticTarget::C, 2)),2),
            0x52 => (Some(Instruction::Bit(ArithmeticTarget::D, 2)),2),
            0x53 => (Some(Instruction::Bit(ArithmeticTarget::E, 2)),2),
            0x54 => (Some(Instruction::Bit(ArithmeticTarget::H, 2)),2),
            0x55 => (Some(Instruction::Bit(ArithmeticTarget::L, 2)),2),
            0x56 => (Some(Instruction::Bit(ArithmeticTarget::AddressHL, 2)),4),
            0x57 => (Some(Instruction::Bit(ArithmeticTarget::A, 2)), 2),

            // BIT instruction (bit 3)
            0x58 => (Some(Instruction::Bit(ArithmeticTarget::B, 3)),2),
            0x59 => (Some(Instruction::Bit(ArithmeticTarget::C, 3)),2),
            0x5A => (Some(Instruction::Bit(ArithmeticTarget::D, 3)),2),
            0x5B => (Some(Instruction::Bit(ArithmeticTarget::E, 3)),2),
            0x5C => (Some(Instruction::Bit(ArithmeticTarget::H, 3)),2),
            0x5D => (Some(Instruction::Bit(ArithmeticTarget::L, 3)),2),
            0x5E => (Some(Instruction::Bit(ArithmeticTarget::AddressHL, 3)),4),
            0x5F => (Some(Instruction::Bit(ArithmeticTarget::A, 3)),2),

            // BIT instruction (bit 4)
            0x60 => (Some(Instruction::Bit(ArithmeticTarget::B, 4)),2),
            0x61 => (Some(Instruction::Bit(ArithmeticTarget::C, 4)),2),
            0x62 => (Some(Instruction::Bit(ArithmeticTarget::D, 4)),2),
            0x63 => (Some(Instruction::Bit(ArithmeticTarget::E, 4)),2),
            0x64 => (Some(Instruction::Bit(ArithmeticTarget::H, 4)),2),
            0x65 => (Some(Instruction::Bit(ArithmeticTarget::L, 4)),2),
            0x66 => (Some(Instruction::Bit(ArithmeticTarget::AddressHL, 4)),4),
            0x67 => (Some(Instruction::Bit(ArithmeticTarget::A, 4)),2),

            // BIT instruction (bit 5)
            0x68 => (Some(Instruction::Bit(ArithmeticTarget::B, 5)),2),
            0x69 => (Some(Instruction::Bit(ArithmeticTarget::C, 5)),2),
            0x6A => (Some(Instruction::Bit(ArithmeticTarget::D, 5)),2),
            0x6B => (Some(Instruction::Bit(ArithmeticTarget::E, 5)),2),
            0x6C => (Some(Instruction::Bit(ArithmeticTarget::H, 5)),2),
            0x6D => (Some(Instruction::Bit(ArithmeticTarget::L, 5)),2),
            0x6E => (Some(Instruction::Bit(ArithmeticTarget::AddressHL, 5)),4),
            0x6F => (Some(Instruction::Bit(ArithmeticTarget::A, 5)),2),

            // BIT instruction (bit 6)
            0x70 => (Some(Instruction::Bit(ArithmeticTarget::B, 6)),2),
            0x71 => (Some(Instruction::Bit(ArithmeticTarget::C, 6)),2),
            0x72 => (Some(Instruction::Bit(ArithmeticTarget::D, 6)),2),
            0x73 => (Some(Instruction::Bit(ArithmeticTarget::E, 6)),2),
            0x74 => (Some(Instruction::Bit(ArithmeticTarget::H, 6)),2),
            0x75 => (Some(Instruction::Bit(ArithmeticTarget::L, 6)),2),
            0x76 => (Some(Instruction::Bit(ArithmeticTarget::AddressHL, 6)),4),
            0x77 => (Some(Instruction::Bit(ArithmeticTarget::A, 6)),2),

            // BIT instruction (bit 7)
            0x78 => (Some(Instruction::Bit(ArithmeticTarget::B, 7)),2),
            0x79 => (Some(Instruction::Bit(ArithmeticTarget::C, 7)),2),
            0x7A => (Some(Instruction::Bit(ArithmeticTarget::D, 7)),2),
            0x7B => (Some(Instruction::Bit(ArithmeticTarget::E, 7)),2),
            0x7C => (Some(Instruction::Bit(ArithmeticTarget::H, 7)),2),
            0x7D => (Some(Instruction::Bit(ArithmeticTarget::L, 7)),2),
            0x7E => (Some(Instruction::Bit(ArithmeticTarget::AddressHL, 7)),4),
            0x7F => (Some(Instruction::Bit(ArithmeticTarget::A, 7)),2),

            // RESET instruction (bit 0)
            0x80 => (Some(Instruction::Reset(ArithmeticTarget::B, 0)),2),
            0x81 => (Some(Instruction::Reset(ArithmeticTarget::C, 0)),2),
            0x82 => (Some(Instruction::Reset(ArithmeticTarget::D, 0)),2),
            0x83 => (Some(Instruction::Reset(ArithmeticTarget::E, 0)),2),
            0x84 => (Some(Instruction::Reset(ArithmeticTarget::H, 0)),2),
            0x85 => (Some(Instruction::Reset(ArithmeticTarget::L, 0)),2),
            0x86 => (Some(Instruction::Reset(ArithmeticTarget::AddressHL, 0)),4),
            0x87 => (Some(Instruction::Reset(ArithmeticTarget::A, 0)),2),

            // RESET instruction (bit 1)
            0x88 => (Some(Instruction::Reset(ArithmeticTarget::B, 1)),2),
            0x89 => (Some(Instruction::Reset(ArithmeticTarget::C, 1)),2),
            0x8A => (Some(Instruction::Reset(ArithmeticTarget::D, 1)),2),
            0x8B => (Some(Instruction::Reset(ArithmeticTarget::E, 1)),2),
            0x8C => (Some(Instruction::Reset(ArithmeticTarget::H, 1)),2),
            0x8D => (Some(Instruction::Reset(ArithmeticTarget::L, 1)),2),
            0x8E => (Some(Instruction::Reset(ArithmeticTarget::AddressHL, 1)),4),
            0x8F => (Some(Instruction::Reset(ArithmeticTarget::A, 1)),2),

            // RESET instruction (bit 2)
            0x90 => (Some(Instruction::Reset(ArithmeticTarget::B, 2)),2),
            0x91 => (Some(Instruction::Reset(ArithmeticTarget::C, 2)),2),
            0x92 => (Some(Instruction::Reset(ArithmeticTarget::D, 2)),2),
            0x93 => (Some(Instruction::Reset(ArithmeticTarget::E, 2)),2),
            0x94 => (Some(Instruction::Reset(ArithmeticTarget::H, 2)),2),
            0x95 => (Some(Instruction::Reset(ArithmeticTarget::L, 2)),2),
            0x96 => (Some(Instruction::Reset(ArithmeticTarget::AddressHL, 2)),4),
            0x97 => (Some(Instruction::Reset(ArithmeticTarget::A, 2)),2),

            // RESET instruction (bit 3)
            0x98 => (Some(Instruction::Reset(ArithmeticTarget::B, 3)),2),
            0x99 => (Some(Instruction::Reset(ArithmeticTarget::C, 3)),2),
            0x9A => (Some(Instruction::Reset(ArithmeticTarget::D, 3)),2),
            0x9B => (Some(Instruction::Reset(ArithmeticTarget::E, 3)),2),
            0x9C => (Some(Instruction::Reset(ArithmeticTarget::H, 3)),2),
            0x9D => (Some(Instruction::Reset(ArithmeticTarget::L, 3)),2),
            0x9E => (Some(Instruction::Reset(ArithmeticTarget::AddressHL, 3)),4),
            0x9F => (Some(Instruction::Reset(ArithmeticTarget::A, 3)),2),

            // RESET instruction (bit 4)
            0xA0 => (Some(Instruction::Reset(ArithmeticTarget::B, 4)),2),
            0xA1 => (Some(Instruction::Reset(ArithmeticTarget::C, 4)),2),
            0xA2 => (Some(Instruction::Reset(ArithmeticTarget::D, 4)),2),
            0xA3 => (Some(Instruction::Reset(ArithmeticTarget::E, 4)),2),
            0xA4 => (Some(Instruction::Reset(ArithmeticTarget::H, 4)),2),
            0xA5 => (Some(Instruction::Reset(ArithmeticTarget::L, 4)),2),
            0xA6 => (Some(Instruction::Reset(ArithmeticTarget::AddressHL, 4)),4),
            0xA7 => (Some(Instruction::Reset(ArithmeticTarget::A, 4)),2),

            // RESET instruction (bit 5)
            0xA8 => (Some(Instruction::Reset(ArithmeticTarget::B, 5)),2),
            0xA9 => (Some(Instruction::Reset(ArithmeticTarget::C, 5)),2),
            0xAA => (Some(Instruction::Reset(ArithmeticTarget::D, 5)),2),
            0xAB => (Some(Instruction::Reset(ArithmeticTarget::E, 5)),2),
            0xAC => (Some(Instruction::Reset(ArithmeticTarget::H, 5)),2),
            0xAD => (Some(Instruction::Reset(ArithmeticTarget::L, 5)),2),
            0xAE => (Some(Instruction::Reset(ArithmeticTarget::AddressHL, 5)),4),
            0xAF => (Some(Instruction::Reset(ArithmeticTarget::A, 5)),2),

            // RESET instruction (bit 6)
            0xB0 => (Some(Instruction::Reset(ArithmeticTarget::B, 6)),2),
            0xB1 => (Some(Instruction::Reset(ArithmeticTarget::C, 6)),2),
            0xB2 => (Some(Instruction::Reset(ArithmeticTarget::D, 6)),2),
            0xB3 => (Some(Instruction::Reset(ArithmeticTarget::E, 6)),2),
            0xB4 => (Some(Instruction::Reset(ArithmeticTarget::H, 6)),2),
            0xB5 => (Some(Instruction::Reset(ArithmeticTarget::L, 6)),2),
            0xB6 => (Some(Instruction::Reset(ArithmeticTarget::AddressHL, 6)),4),
            0xB7 => (Some(Instruction::Reset(ArithmeticTarget::A, 6)),2),

            // RESET instruction (bit 7)
            0xB8 => (Some(Instruction::Reset(ArithmeticTarget::B, 7)),2),
            0xB9 => (Some(Instruction::Reset(ArithmeticTarget::C, 7)),2),
            0xBA => (Some(Instruction::Reset(ArithmeticTarget::D, 7)),2),
            0xBB => (Some(Instruction::Reset(ArithmeticTarget::E, 7)),2),
            0xBC => (Some(Instruction::Reset(ArithmeticTarget::H, 7)),2),
            0xBD => (Some(Instruction::Reset(ArithmeticTarget::L, 7)),2),
            0xBE => (Some(Instruction::Reset(ArithmeticTarget::AddressHL, 7)),4),
            0xBF => (Some(Instruction::Reset(ArithmeticTarget::A, 7)),2),

            // SET instruction (bit 0)
            0xC0 => (Some(Instruction::Set(ArithmeticTarget::B, 0)),2),
            0xC1 => (Some(Instruction::Set(ArithmeticTarget::C, 0)),2),
            0xC2 => (Some(Instruction::Set(ArithmeticTarget::D, 0)),2),
            0xC3 => (Some(Instruction::Set(ArithmeticTarget::E, 0)),2),
            0xC4 => (Some(Instruction::Set(ArithmeticTarget::H, 0)),2),
            0xC5 => (Some(Instruction::Set(ArithmeticTarget::L, 0)),2),
            0xC6 => (Some(Instruction::Set(ArithmeticTarget::AddressHL, 0)),4),
            0xC7 => (Some(Instruction::Set(ArithmeticTarget::A, 0)),2),

            // SET instruction (bit 1)
            0xC8 => (Some(Instruction::Set(ArithmeticTarget::B, 1)),2),
            0xC9 => (Some(Instruction::Set(ArithmeticTarget::C, 1)),2),
            0xCA => (Some(Instruction::Set(ArithmeticTarget::D, 1)),2),
            0xCB => (Some(Instruction::Set(ArithmeticTarget::E, 1)),2),
            0xCC => (Some(Instruction::Set(ArithmeticTarget::H, 1)),2),
            0xCD => (Some(Instruction::Set(ArithmeticTarget::L, 1)),2),
            0xCE => (Some(Instruction::Set(ArithmeticTarget::AddressHL, 1)),4),
            0xCF => (Some(Instruction::Set(ArithmeticTarget::A, 1)),2),

            // SET instruction (bit 2)
            0xD0 => (Some(Instruction::Set(ArithmeticTarget::B, 2)),2),
            0xD1 => (Some(Instruction::Set(ArithmeticTarget::C, 2)),2),
            0xD2 => (Some(Instruction::Set(ArithmeticTarget::D, 2)),2),
            0xD3 => (Some(Instruction::Set(ArithmeticTarget::E, 2)),2),
            0xD4 => (Some(Instruction::Set(ArithmeticTarget::H, 2)),2),
            0xD5 => (Some(Instruction::Set(ArithmeticTarget::L, 2)),2),
            0xD6 => (Some(Instruction::Set(ArithmeticTarget::AddressHL, 2)),4),
            0xD7 => (Some(Instruction::Set(ArithmeticTarget::A, 2)),2),

            // SET instruction (bit 3)
            0xD8 => (Some(Instruction::Set(ArithmeticTarget::B, 3)),2),
            0xD9 => (Some(Instruction::Set(ArithmeticTarget::C, 3)),2),
            0xDA => (Some(Instruction::Set(ArithmeticTarget::D, 3)),2),
            0xDB => (Some(Instruction::Set(ArithmeticTarget::E, 3)),2),
            0xDC => (Some(Instruction::Set(ArithmeticTarget::H, 3)),2),
            0xDD => (Some(Instruction::Set(ArithmeticTarget::L, 3)),2),
            0xDE => (Some(Instruction::Set(ArithmeticTarget::AddressHL, 3)),4),
            0xDF => (Some(Instruction::Set(ArithmeticTarget::A, 3)),2),

            // SET instruction (bit 4)
            0xE0 => (Some(Instruction::Set(ArithmeticTarget::B, 4)),2),
            0xE1 => (Some(Instruction::Set(ArithmeticTarget::C, 4)),2),
            0xE2 => (Some(Instruction::Set(ArithmeticTarget::D, 4)),2),
            0xE3 => (Some(Instruction::Set(ArithmeticTarget::E, 4)),2),
            0xE4 => (Some(Instruction::Set(ArithmeticTarget::H, 4)),2),
            0xE5 => (Some(Instruction::Set(ArithmeticTarget::L, 4)),2),
            0xE6 => (Some(Instruction::Set(ArithmeticTarget::AddressHL, 4)),4),
            0xE7 => (Some(Instruction::Set(ArithmeticTarget::A, 4)),2),

            // SET instruction (bit 5)
            0xE8 => (Some(Instruction::Set(ArithmeticTarget::B, 5)),2),
            0xE9 => (Some(Instruction::Set(ArithmeticTarget::C, 5)),2),
            0xEA => (Some(Instruction::Set(ArithmeticTarget::D, 5)),2),
            0xEB => (Some(Instruction::Set(ArithmeticTarget::E, 5)),2),
            0xEC => (Some(Instruction::Set(ArithmeticTarget::H, 5)),2),
            0xED => (Some(Instruction::Set(ArithmeticTarget::L, 5)),2),
            0xEE => (Some(Instruction::Set(ArithmeticTarget::AddressHL, 5)),4),
            0xEF => (Some(Instruction::Set(ArithmeticTarget::A, 5)),2),

            // SET instruction (bit 6)
            0xF0 => (Some(Instruction::Set(ArithmeticTarget::B, 6)),2),
            0xF1 => (Some(Instruction::Set(ArithmeticTarget::C, 6)),2),
            0xF2 => (Some(Instruction::Set(ArithmeticTarget::D, 6)),2),
            0xF3 => (Some(Instruction::Set(ArithmeticTarget::E, 6)),2),
            0xF4 => (Some(Instruction::Set(ArithmeticTarget::H, 6)),2),
            0xF5 => (Some(Instruction::Set(ArithmeticTarget::L, 6)),2),
            0xF6 => (Some(Instruction::Set(ArithmeticTarget::AddressHL, 6)),4),
            0xF7 => (Some(Instruction::Set(ArithmeticTarget::A, 6)),2),

            // SET instruction (bit 7)
            0xF8 => (Some(Instruction::Set(ArithmeticTarget::B, 7)),2),
            0xF9 => (Some(Instruction::Set(ArithmeticTarget::C, 7)),2),
            0xFA => (Some(Instruction::Set(ArithmeticTarget::D, 7)),2),
            0xFB => (Some(Instruction::Set(ArithmeticTarget::E, 7)),2),
            0xFC => (Some(Instruction::Set(ArithmeticTarget::H, 7)),2),
            0xFD => (Some(Instruction::Set(ArithmeticTarget::L, 7)),2),
            0xFE => (Some(Instruction::Set(ArithmeticTarget::AddressHL, 7)),4),
            0xFF => (Some(Instruction::Set(ArithmeticTarget::A, 7)),2),
            //_ => panic!("unknown not prefixed instruction")
        }
    }

    pub fn from_byte_not_prefixed(byte: u8) -> (Option<Instruction>,u8) {
        match byte {
            0x00 => (Some(Instruction::Nop),1),
            0x01 => (Some(Instruction::Ld(LoadType::Word(LoadWordTarget::BC, LoadWordSource::D16))),3),
            0x02 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressBC, LoadByteSource::A))),2),
            0x03 => (Some(Instruction::Inc(ArithmeticTarget::BC)),2),
            0x04 => (Some(Instruction::Inc(ArithmeticTarget::B)),1),
            0x05 => (Some(Instruction::Dec(ArithmeticTarget::B)),1),
            0x06 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))),2),
            0x07 => (Some(Instruction::Rlca),1),
            0x08 => (Some(Instruction::Ld(LoadType::Word(LoadWordTarget::Address16, LoadWordSource::SP))),5),
            0x09 => (Some(Instruction::Addhl(ArithmeticTarget::BC)),2),
            0x0A => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressBC))),2),
            0x0B => (Some(Instruction::Dec(ArithmeticTarget::BC)),2),
            0x0C => (Some(Instruction::Inc(ArithmeticTarget::C)),1),
            0x0D => (Some(Instruction::Dec(ArithmeticTarget::C)),1),
            0x0E => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))),2),
            0x0F => (Some(Instruction::Rrca),1),

            0x10 => (Some(Instruction::Stop),1),
            0x11 => (Some(Instruction::Ld(LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16))),3),
            0x12 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressDE, LoadByteSource::A))),2),
            0x13 => (Some(Instruction::Inc(ArithmeticTarget::DE)),2),
            0x14 => (Some(Instruction::Inc(ArithmeticTarget::D)),1),
            0x15 => (Some(Instruction::Dec(ArithmeticTarget::D)),1),
            0x16 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))),2),
            0x17 => (Some(Instruction::Rla),1),
            0x18 => (Some(Instruction::Jr(JumpTest::Always)),2), // Remplacez "offset" par la valeur appropriée.
            0x19 => (Some(Instruction::Addhl(ArithmeticTarget::DE)),2),
            0x1A => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressDE))),2),
            0x1B => (Some(Instruction::Dec(ArithmeticTarget::DE)),2),
            0x1C => (Some(Instruction::Inc(ArithmeticTarget::E)),1),
            0x1D => (Some(Instruction::Dec(ArithmeticTarget::E)),1),
            0x1E => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))),2),
            0x1F => (Some(Instruction::Rra),1),
                                                                                                                                        
            0x20 => (Some(Instruction::Jr(JumpTest::NotZero)),2), // Remplacez "offset" par la valeur appropriée.
            0x21 => (Some(Instruction::Ld(LoadType::Word(LoadWordTarget::HL, LoadWordSource::D16))),3),
            0x22 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressHLP, LoadByteSource::A))),2),
            0x23 => (Some(Instruction::Inc(ArithmeticTarget::HL)),2),
            0x24 => (Some(Instruction::Inc(ArithmeticTarget::H)),1),
            0x25 => (Some(Instruction::Dec(ArithmeticTarget::H)),1),
            0x26 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))),2),
            0x27 => (Some(Instruction::Daa),1),
            0x28 => (Some(Instruction::Jr(JumpTest::Zero)),2), // Remplacez "offset" par la valeur appropriée.
            0x29 => (Some(Instruction::Addhl(ArithmeticTarget::HL)),2),
            0x2A => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressHLP))),2),
            0x2B => (Some(Instruction::Dec(ArithmeticTarget::HL)),2),
            0x2C => (Some(Instruction::Inc(ArithmeticTarget::L)),1),
            0x2D => (Some(Instruction::Dec(ArithmeticTarget::L)),1),
            0x2E => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))),2),
            0x2F => (Some(Instruction::Cpl),1),
                                                                                                                                                                                                        
            0x30 => (Some(Instruction::Jr(JumpTest::NotCarry)),2), // Remplacez "offset" par la valeur appropriée.
            0x31 => (Some(Instruction::Ld(LoadType::Word(LoadWordTarget::SP, LoadWordSource::D16))),3),
            0x32 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressHLM, LoadByteSource::A))),2),
            0x33 => (Some(Instruction::Inc(ArithmeticTarget::SP)),2),
            0x34 => (Some(Instruction::Inc(ArithmeticTarget::AddressHL)),3),
            0x35 => (Some(Instruction::Dec(ArithmeticTarget::AddressHL)),3),
            0x36 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::D8))),3),
            0x37 => (Some(Instruction::Scf),1),
            0x38 => (Some(Instruction::Jr(JumpTest::Carry)),2), // Remplacez "offset" par la valeur appropriée.
            0x39 => (Some(Instruction::Addhl(ArithmeticTarget::SP)),2),
            0x3A => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressHLM))),2),
            0x3B => (Some(Instruction::Dec(ArithmeticTarget::SP)),2),
            0x3C => (Some(Instruction::Inc(ArithmeticTarget::A)),1),
            0x3D => (Some(Instruction::Dec(ArithmeticTarget::A)),1),
            0x3E => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))),2),
            0x3F => (Some(Instruction::Ccf),1),
                                                                                                                                                                                                                                                                        
            0x40 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))),1),
            0x41 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))),1),
            0x42 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))),1),
            0x43 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))),1),
            0x44 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))),1),
            0x45 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))),1),
            0x46 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::AddressHL))),2),
            0x47 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))),1),
            0x48 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))),1),
            0x49 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))),1),
            0x4A => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))),1),
            0x4B => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))),1),
            0x4C => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))),1),
            0x4D => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))),1),
            0x4E => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::AddressHL))),2),
            0x4F => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))),1),
                                                                                                                                                                                                                                                                                                                                        
            0x50 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))),1),
            0x51 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))),1),
            0x52 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))),1),
            0x53 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))),1),
            0x54 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))),1),
            0x55 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))),1),
            0x56 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::AddressHL))),2),
            0x57 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))),1),
            0x58 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))),1),
            0x59 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))),1),
            0x5A => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))),1),
            0x5B => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))),1),
            0x5C => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))),1),
            0x5D => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))),1),
            0x5E => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::AddressHL))),2),
            0x5F => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))),1),
                                                                                                                                                                                                                                                                                                                                                                                                        
            0x60 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))),1),
            0x61 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))),1),
            0x62 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))),1),
            0x63 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))),1),
            0x64 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))),1),
            0x65 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))),1),
            0x66 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::AddressHL))),2),
            0x67 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))),1),
            0x68 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))),1),
            0x69 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))),1),
            0x6A => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))),1),
            0x6B => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))),1),
            0x6C => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))),1),
            0x6D => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))),1),
            0x6E => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::AddressHL))),2),
            0x6F => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0x70 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::B))),2),
            0x71 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::C))),2),
            0x72 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::D))),2),
            0x73 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::E))),2),
            0x74 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::H))),2),
            0x75 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::L))),2),
            0x76 => (Some(Instruction::Halt),1),
            0x77 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressHL, LoadByteSource::A))),2),
            0x78 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))),1),
            0x79 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))),1),
            0x7A => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))),1),
            0x7B => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))),1),
            0x7C => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))),1),
            0x7D => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))),1),
            0x7E => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressHL))),2),
            0x7F => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0x80 => (Some(Instruction::Add(ArithmeticTarget::B)),1),
            0x81 => (Some(Instruction::Add(ArithmeticTarget::C)),1),
            0x82 => (Some(Instruction::Add(ArithmeticTarget::D)),1),
            0x83 => (Some(Instruction::Add(ArithmeticTarget::E)),1),
            0x84 => (Some(Instruction::Add(ArithmeticTarget::H)),1),
            0x85 => (Some(Instruction::Add(ArithmeticTarget::L)),1),
            0x86 => (Some(Instruction::Add(ArithmeticTarget::AddressHL)),2),
            0x87 => (Some(Instruction::Add(ArithmeticTarget::A)),1),
            0x88 => (Some(Instruction::Adc(ArithmeticTarget::B)),1),
            0x89 => (Some(Instruction::Adc(ArithmeticTarget::C)),1),
            0x8A => (Some(Instruction::Adc(ArithmeticTarget::D)),1),
            0x8B => (Some(Instruction::Adc(ArithmeticTarget::E)),1),
            0x8C => (Some(Instruction::Adc(ArithmeticTarget::H)),1),
            0x8D => (Some(Instruction::Adc(ArithmeticTarget::L)),1),
            0x8E => (Some(Instruction::Adc(ArithmeticTarget::AddressHL)),2),
            0x8F => (Some(Instruction::Adc(ArithmeticTarget::A)),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0x90 => (Some(Instruction::Sub(ArithmeticTarget::B)),1),
            0x91 => (Some(Instruction::Sub(ArithmeticTarget::C)),1),
            0x92 => (Some(Instruction::Sub(ArithmeticTarget::D)),1),
            0x93 => (Some(Instruction::Sub(ArithmeticTarget::E)),1),
            0x94 => (Some(Instruction::Sub(ArithmeticTarget::H)),1),
            0x95 => (Some(Instruction::Sub(ArithmeticTarget::L)),1),
            0x96 => (Some(Instruction::Sub(ArithmeticTarget::AddressHL)),2),
            0x97 => (Some(Instruction::Sub(ArithmeticTarget::A)),1),
            0x98 => (Some(Instruction::Sbc(ArithmeticTarget::B)),1),
            0x99 => (Some(Instruction::Sbc(ArithmeticTarget::C)),1),
            0x9A => (Some(Instruction::Sbc(ArithmeticTarget::D)),1),
            0x9B => (Some(Instruction::Sbc(ArithmeticTarget::E)),1),
            0x9C => (Some(Instruction::Sbc(ArithmeticTarget::H)),1),
            0x9D => (Some(Instruction::Sbc(ArithmeticTarget::L)),1),
            0x9E => (Some(Instruction::Sbc(ArithmeticTarget::AddressHL)),2),
            0x9F => (Some(Instruction::Sbc(ArithmeticTarget::A)),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xA0 => (Some(Instruction::And(ArithmeticTarget::B)),1),
            0xA1 => (Some(Instruction::And(ArithmeticTarget::C)),1),
            0xA2 => (Some(Instruction::And(ArithmeticTarget::D)),1),
            0xA3 => (Some(Instruction::And(ArithmeticTarget::E)),1),
            0xA4 => (Some(Instruction::And(ArithmeticTarget::H)),1),
            0xA5 => (Some(Instruction::And(ArithmeticTarget::L)),1),
            0xA6 => (Some(Instruction::And(ArithmeticTarget::AddressHL)),2),
            0xA7 => (Some(Instruction::And(ArithmeticTarget::A)),1),
            0xA8 => (Some(Instruction::Xor(ArithmeticTarget::B)),1),
            0xA9 => (Some(Instruction::Xor(ArithmeticTarget::C)),1),
            0xAA => (Some(Instruction::Xor(ArithmeticTarget::D)),1),
            0xAB => (Some(Instruction::Xor(ArithmeticTarget::E)),1),
            0xAC => (Some(Instruction::Xor(ArithmeticTarget::H)),1),
            0xAD => (Some(Instruction::Xor(ArithmeticTarget::L)),1),
            0xAE => (Some(Instruction::Xor(ArithmeticTarget::AddressHL)),2),
            0xAF => (Some(Instruction::Xor(ArithmeticTarget::A)),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xB0 => (Some(Instruction::Or(ArithmeticTarget::B)),1),
            0xB1 => (Some(Instruction::Or(ArithmeticTarget::C)),1),
            0xB2 => (Some(Instruction::Or(ArithmeticTarget::D)),1),
            0xB3 => (Some(Instruction::Or(ArithmeticTarget::E)),1),
            0xB4 => (Some(Instruction::Or(ArithmeticTarget::H)),1),
            0xB5 => (Some(Instruction::Or(ArithmeticTarget::L)),1),
            0xB6 => (Some(Instruction::Or(ArithmeticTarget::AddressHL)),2),
            0xB7 => (Some(Instruction::Or(ArithmeticTarget::A)),1),
            0xB8 => (Some(Instruction::Cp(ArithmeticTarget::B)),1),
            0xB9 => (Some(Instruction::Cp(ArithmeticTarget::C)),1),
            0xBA => (Some(Instruction::Cp(ArithmeticTarget::D)),1),
            0xBB => (Some(Instruction::Cp(ArithmeticTarget::E)),1),
            0xBC => (Some(Instruction::Cp(ArithmeticTarget::H)),1),
            0xBD => (Some(Instruction::Cp(ArithmeticTarget::L)),1),
            0xBE => (Some(Instruction::Cp(ArithmeticTarget::AddressHL)),2),
            0xBF => (Some(Instruction::Cp(ArithmeticTarget::A)),1),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xC0 => (Some(Instruction::Ret(JumpTest::NotZero)),2),
            0xC1 => (Some(Instruction::Pop(StackTarget::BC)),3),
            0xC2 => (Some(Instruction::Jp(JumpTest::NotZero, JumpCondition::Address16)),3),
            0xC3 => (Some(Instruction::Jp(JumpTest::Always, JumpCondition::Address16)),3),
            0xC4 => (Some(Instruction::Call(JumpTest::NotZero)),3),
            0xC5 => (Some(Instruction::Push(StackTarget::BC)),4),
            0xC6 => (Some(Instruction::Add(ArithmeticTarget::D8)),2),
            0xC7 => (Some(Instruction::Rst(RstTarget::Rst00H)),4),
            0xC8 => (Some(Instruction::Ret(JumpTest::Zero)),2),
            0xC9 => (Some(Instruction::Ret(JumpTest::Always)),1),
            0xCA => (Some(Instruction::Jp(JumpTest::Zero, JumpCondition::Address16)),3),
            0xCB => (Some(Instruction::PrefixCB),1),
            0xCC => (Some(Instruction::Call(JumpTest::Zero)),3),
            0xCD => (Some(Instruction::Call(JumpTest::Always)),3),
            0xCE => (Some(Instruction::Adc(ArithmeticTarget::D8)),2),
            0xCF => (Some(Instruction::Rst(RstTarget::Rst08H)),4),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xD0 => (Some(Instruction::Ret(JumpTest::NotCarry)),2),
            0xD1 => (Some(Instruction::Pop(StackTarget::DE)),3),
            0xD2 => (Some(Instruction::Jp(JumpTest::NotCarry, JumpCondition::Address16)),3),
            0xD4 => (Some(Instruction::Call(JumpTest::NotCarry)),3),
            0xD5 => (Some(Instruction::Push(StackTarget::DE)),4),
            0xD6 => (Some(Instruction::Sub(ArithmeticTarget::D8)),2),
            0xD7 => (Some(Instruction::Rst(RstTarget::Rst10H)),4),
            0xD8 => (Some(Instruction::Ret(JumpTest::Carry)),2),
            0xD9 => (Some(Instruction::Reti),4),
            0xDA => (Some(Instruction::Jp(JumpTest::Carry, JumpCondition::Address16)),3),
            0xDC => (Some(Instruction::Call(JumpTest::Carry)),3),
            0xDE => (Some(Instruction::Sbc(ArithmeticTarget::D8)),2),
            0xDF => (Some(Instruction::Rst(RstTarget::Rst18H)),4),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xE0 => (Some(Instruction::Ldh(LoadType::Byte(LoadByteTarget::Address8, LoadByteSource::A))),3),
            0xE1 => (Some(Instruction::Pop(StackTarget::HL)),3),
            0xE2 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::AddressC, LoadByteSource::A))),2),
            0xE5 => (Some(Instruction::Push(StackTarget::HL)),4),
            0xE6 => (Some(Instruction::And(ArithmeticTarget::D8)),2),
            0xE7 => (Some(Instruction::Rst(RstTarget::Rst20H)),4),
            0xE8 => (Some(Instruction::Addsp()),4),
            0xE9 => (Some(Instruction::Jp(JumpTest::Always, JumpCondition::AddressHL)),1),
            0xEA => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::Address16, LoadByteSource::A))),4),
            0xEE => (Some(Instruction::Xor(ArithmeticTarget::D8)),2),
            0xEF => (Some(Instruction::Rst(RstTarget::Rst28H)),4),
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
            0xF0 => (Some(Instruction::Ldh(LoadType::Byte(LoadByteTarget::A, LoadByteSource::Address8))),3),
            0xF1 => (Some(Instruction::Pop(StackTarget::AF)),3),
            0xF2 => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressC))),2),
            0xF3 => (Some(Instruction::Di),1),
            0xF5 => (Some(Instruction::Push(StackTarget::AF)),4),
            0xF6 => (Some(Instruction::Or(ArithmeticTarget::D8)),2),
            0xF7 => (Some(Instruction::Rst(RstTarget::Rst30H)),4),
            0xF8 => (Some(Instruction::Ld(LoadType::Word(LoadWordTarget::HL, LoadWordSource::SPR8))),3),
            0xF9 => (Some(Instruction::Ld(LoadType::Word(LoadWordTarget::SP, LoadWordSource::HL))),2),
            0xFA => (Some(Instruction::Ld(LoadType::Byte(LoadByteTarget::A, LoadByteSource::Address16))),4),
            0xFB => (Some(Instruction::Ei),1),
            0xFE => (Some(Instruction::Cp(ArithmeticTarget::D8)),2),
            0xFF => (Some(Instruction::Rst(RstTarget::Rst38H)),4),
            _ => panic!("unknown prefixed instruction")
        }
    }
}
