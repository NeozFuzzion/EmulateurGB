pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

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
    RST(RstTarget),
    PrefixCB
}

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L, HL, AF, BC, DE, SP, AddressHL, D8
}


impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        
        if prefixed {
            Self::print_debug_prefixed(byte.clone());
            
            Instruction::from_byte_prefixed(byte)
        } else {
            Self::print_debug(byte.clone());
            Instruction::from_byte_not_prefixed(byte)
        }
    }


    pub fn print_debug(byte: u8) {
        print!("0x{:02X} ", byte);
        match byte {
            0x00 => println!("NOP"), 0x01 => println!("LD BC, d16"), 0x02 => println!("LD (BC), A"), 0x03 => println!("INC BC"), 0x04 => println!("INC B"), 0x05 => println!("DEC B"), 0x06 => println!("LD B, d8"), 0x07 => println!("RLCA"), 0x08 => println!("LD (a16), SP"), 0x09 => println!("ADD HL, BC"), 0x0A => println!("LD A, (BC)"), 0x0B => println!("DEC BC"), 0x0C => println!("INC C"), 0x0D => println!("DEC C"), 0x0E => println!("LD C, d8"), 0x0F => println!("RRCA"),
            0x10 => println!("STOP"), 0x11 => println!("LD DE, d16"), 0x12 => println!("LD (DE), A"), 0x13 => println!("INC DE"), 0x14 => println!("INC D"), 0x15 => println!("DEC D"), 0x16 => println!("LD D, d8"), 0x17 => println!("RLA"), 0x18 => println!("JR r8"), 0x19 => println!("ADD HL, DE"), 0x1A => println!("LD A, (DE)"), 0x1B => println!("DEC DE"), 0x1C => println!("INC E"), 0x1D => println!("DEC E"), 0x1E => println!("LD E, d8"), 0x1F => println!("RRA"),
            0x20 => println!("JR NZ, r8"), 0x21 => println!("LD HL, d16"), 0x22 => println!("LDI (HL), A"), 0x23 => println!("INC HL"), 0x24 => println!("INC H"), 0x25 => println!("DEC H"), 0x26 => println!("LD H, d8"), 0x27 => println!("DAA"), 0x28 => println!("JR Z, r8"), 0x29 => println!("ADD HL, HL"), 0x2A => println!("LDI A, (HL)"), 0x2B => println!("DEC HL"), 0x2C => println!("INC L"), 0x2D => println!("DEC L"), 0x2E => println!("LD L, d8"), 0x2F => println!("CPL"),
            0x30 => println!("JR NC, r8"), 0x31 => println!("LD SP, d16"), 0x32 => println!("LDD (HL), A"), 0x33 => println!("INC SP"), 0x34 => println!("INC (HL)"), 0x35 => println!("DEC (HL)"), 0x36 => println!("LD (HL), d8"), 0x37 => println!("SCF"), 0x38 => println!("JR C, r8"), 0x39 => println!("ADD HL, SP"), 0x3A => println!("LDD A, (HL)"), 0x3B => println!("DEC SP"), 0x3C => println!("INC A"), 0x3D => println!("DEC A"), 0x3E => println!("LD A, d8"), 0x3F => println!("CCF"),
            0x40 => println!("LD B, B"), 0x41 => println!("LD B, C"), 0x42 => println!("LD B, D"), 0x43 => println!("LD B, E"), 0x44 => println!("LD B, H"), 0x45 => println!("LD B, L"), 0x46 => println!("LD B, (HL)"), 0x47 => println!("LD B, A"), 0x48 => println!("LD C, B"), 0x49 => println!("LD C, C"), 0x4A => println!("LD C, D"), 0x4B => println!("LD C, E"), 0x4C => println!("LD C, H"), 0x4D => println!("LD C, L"), 0x4E => println!("LD C, (HL)"), 0x4F => println!("LD C, A"),
            0x50 => println!("LD D, B"), 0x51 => println!("LD D, C"), 0x52 => println!("LD D, D"), 0x53 => println!("LD D, E"), 0x54 => println!("LD D, H"), 0x55 => println!("LD D, L"), 0x56 => println!("LD D, (HL)"), 0x57 => println!("LD D, A"), 0x58 => println!("LD E, B"), 0x59 => println!("LD E, C"), 0x5A => println!("LD E, D"), 0x5B => println!("LD E, E"), 0x5C => println!("LD E, H"), 0x5D => println!("LD E, L"), 0x5E => println!("LD E, (HL)"), 0x5F => println!("LD E, A"),
            0x60 => println!("LD H, B"), 0x61 => println!("LD H, C"), 0x62 => println!("LD H, D"), 0x63 => println!("LD H, E"), 0x64 => println!("LD H, H"), 0x65 => println!("LD H, L"), 0x66 => println!("LD H, (HL)"), 0x67 => println!("LD H, A"), 0x68 => println!("LD L, B"), 0x69 => println!("LD L, C"), 0x6A => println!("LD L, D"), 0x6B => println!("LD L, E"), 0x6C => println!("LD L, H"), 0x6D => println!("LD L, L"), 0x6E => println!("LD L, (HL)"), 0x6F => println!("LD L, A"),
            0x70 => println!("LD (HL), B"), 0x71 => println!("LD (HL), C"), 0x72 => println!("LD (HL), D"), 0x73 => println!("LD (HL), E"), 0x74 => println!("LD (HL), H"), 0x75 => println!("LD (HL), L"), 0x76 => println!("HALT"), 0x77 => println!("LD (HL), A"), 0x78 => println!("LD A, B"), 0x79 => println!("LD A, C"), 0x7A => println!("LD A, D"), 0x7B => println!("LD A, E"), 0x7C => println!("LD A, H"), 0x7D => println!("LD A, L"), 0x7E => println!("LD A, (HL)"), 0x7F => println!("LD A, A"),
            0x80 => println!("ADD A, B"), 0x81 => println!("ADD A, C"), 0x82 => println!("ADD A, D"), 0x83 => println!("ADD A, E"), 0x84 => println!("ADD A, H"), 0x85 => println!("ADD A, L"), 0x86 => println!("ADD A, (HL)"), 0x87 => println!("ADD A, A"), 0x88 => println!("ADC A, B"), 0x89 => println!("ADC A, C"), 0x8A => println!("ADC A, D"), 0x8B => println!("ADC A, E"), 0x8C => println!("ADC A, H"), 0x8D => println!("ADC A, L"), 0x8E => println!("ADC A, (HL)"), 0x8F => println!("ADC A, A"),
            0x90 => println!("SUB B"), 0x91 => println!("SUB C"), 0x92 => println!("SUB D"), 0x93 => println!("SUB E"), 0x94 => println!("SUB H"), 0x95 => println!("SUB L"), 0x96 => println!("SUB (HL)"), 0x97 => println!("SUB A"), 0x98 => println!("SBC A, B"), 0x99 => println!("SBC A, C"), 0x9A => println!("SBC A, D"), 0x9B => println!("SBC A, E"), 0x9C => println!("SBC A, H"), 0x9D => println!("SBC A, L"), 0x9E => println!("SBC A, (HL)"), 0x9F => println!("SBC A, A"),
            0xA0 => println!("AND B"), 0xA1 => println!("AND C"), 0xA2 => println!("AND D"), 0xA3 => println!("AND E"), 0xA4 => println!("AND H"), 0xA5 => println!("AND L"), 0xA6 => println!("AND (HL)"), 0xA7 => println!("AND A"), 0xA8 => println!("XOR B"), 0xA9 => println!("XOR C"), 0xAA => println!("XOR D"), 0xAB => println!("XOR E"), 0xAC => println!("XOR H"), 0xAD => println!("XOR L"), 0xAE => println!("XOR (HL)"), 0xAF => println!("XOR A"),
            0xB0 => println!("OR B"), 0xB1 => println!("OR C"), 0xB2 => println!("OR D"), 0xB3 => println!("OR E"), 0xB4 => println!("OR H"), 0xB5 => println!("OR L"), 0xB6 => println!("OR (HL)"), 0xB7 => println!("OR A"), 0xB8 => println!("CP B"), 0xB9 => println!("CP C"), 0xBA => println!("CP D"), 0xBB => println!("CP E"), 0xBC => println!("CP H"), 0xBD => println!("CP L"), 0xBE => println!("CP (HL)"), 0xBF => println!("CP A"),
            0xC0 => println!("RET NZ"), 0xC1 => println!("POP BC"), 0xC2 => println!("JP NZ, a16"), 0xC3 => println!("JP a16"), 0xC4 => println!("CALL NZ, a16"), 0xC5 => println!("PUSH BC"), 0xC6 => println!("ADD A, d8"), 0xC7 => println!("RST 00H"), 0xC8 => println!("RET Z"), 0xC9 => println!("RET"), 0xCA => println!("JP Z, a16"), 0xCC => println!("CALL Z, a16"), 0xCD => println!("CALL a16"), 0xCE => println!("ADC A, d8"), 0xCF => println!("RST 08H"),
            0xD0 => println!("RET NC"), 0xD1 => println!("POP DE"), 0xD2 => println!("JP NC, a16"), 0xD4 => println!("CALL NC, a16"), 0xD5 => println!("PUSH DE"), 0xD6 => println!("SUB d8"), 0xD7 => println!("RST 10H"), 0xD8 => println!("RET C"), 0xD9 => println!("RETI"), 0xDA => println!("JP C, a16"), 0xDC => println!("CALL C, a16"), 0xDE => println!("SBC A, d8"), 0xDF => println!("RST 18H"),
            0xE0 => println!("LDH (a8), A"), 0xE1 => println!("POP HL"), 0xE2 => println!("LD (C), A"), 0xE5 => println!("PUSH HL"), 0xE6 => println!("AND d8"), 0xE7 => println!("RST 20H"), 0xE8 => println!("ADD SP, r8"), 0xE9 => println!("JP (HL)"), 0xEA => println!("LD (a16), A"), 0xEE => println!("XOR d8"), 0xEF => println!("RST 28H"),
            0xF0 => println!("LDH A, (a8)"), 0xF1 => println!("POP AF"), 0xF2 => println!("LD A, (C)"), 0xF3 => println!("DI"), 0xF5 => println!("PUSH AF"), 0xF6 => println!("OR d8"), 0xF7 => println!("RST 30H"), 0xF8 => println!("LD HL, SP+r8"), 0xF9 => println!("LD SP, HL"), 0xFA => println!("LD A, (a16)"), 0xFB => println!("EI"), 0xFE => println!("CP d8"), 0xFF => println!("RST 38H"),
            _ => {}
        }
    }

    pub fn print_debug_prefixed(byte: u8) {
        match byte {
            0x00 => println!("Prefix: RLC B"), 0x01 => println!("Prefix: RLC C"), 0x02 => println!("Prefix: RLC D"), 0x03 => println!("Prefix: RLC E"), 0x04 => println!("Prefix: RLC H"), 0x05 => println!("Prefix: RLC L"), 0x06 => println!("Prefix: RLC (HL)"), 0x07 => println!("Prefix: RLC A"), 0x08 => println!("Prefix: RRC B"), 0x09 => println!("Prefix: RRC C"), 0x0a => println!("Prefix: RRC D"), 0x0b => println!("Prefix: RRC E"), 0x0c => println!("Prefix: RRC H"), 0x10 => println!("Prefix: RL B"), 0x11 => println!("Prefix: RL C"), 0x12 => println!("Prefix: RL D"),
            0x13 => println!("Prefix: RL E"), 0x14 => println!("Prefix: RL H"), 0x15 => println!("Prefix: RL L"), 0x16 => println!("Prefix: RL (HL)"), 0x17 => println!("Prefix: RL A"), 0x18 => println!("Prefix: RR B"), 0x19 => println!("Prefix: RR C"), 0x1a => println!("Prefix: RR D"), 0x1b => println!("Prefix: RR E"), 0x0d => println!("Prefix: Prefix: RRC L"), 0x0e => println!("Prefix: Prefix: RRC (HL)"), 0x0f => println!("Prefix: Prefix: RRC A"), 0x1c => println!("Prefix: RR H"), 0x1d => println!("Prefix: RR L"), 0x1e => println!("Prefix: RR (HL)"), 0x1f => println!("Prefix: RR A"),
            0x20 => println!("Prefix: SLA B"), 0x21 => println!("Prefix: SLA C"), 0x22 => println!("Prefix: SLA D"), 0x23 => println!("Prefix: SLA E"), 0x24 => println!("Prefix: SLA H"), 0x25 => println!("Prefix: SLA L"), 0x26 => println!("Prefix: SLA (HL)"), 0x27 => println!("Prefix: SLA A"), 0x28 => println!("Prefix: SRA B"), 0x29 => println!("Prefix: SRA C"), 0x2a => println!("Prefix: SRA D"), 0x2b => println!("Prefix: SRA E"), 0x2c => println!("Prefix: SRA H"), 0x2d => println!("Prefix: SRA L"), 0x2e => println!("Prefix: SRA (HL)"), 0x2f => println!("Prefix: SRA A"),
            0x30 => println!("Prefix: SWAP B"), 0x31 => println!("Prefix: SWAP C"), 0x32 => println!("Prefix: SWAP D"), 0x33 => println!("Prefix: SWAP E"), 0x34 => println!("Prefix: SWAP H"), 0x35 => println!("Prefix: SWAP L"), 0x36 => println!("Prefix: SWAP (HL)"), 0x37 => println!("Prefix: SWAP A"), 0x38 => println!("Prefix: SRL B"), 0x39 => println!("Prefix: SRL C"), 0x3a => println!("Prefix: SRL D"), 0x3b => println!("Prefix: SRL E"), 0x3c => println!("Prefix: SRL H"), 0x3d => println!("Prefix: SRL L"), 0x3e => println!("Prefix: SRL (HL)"), 0x3f => println!("Prefix: SRL A"),
            0x40 => println!("Prefix: BIT 0, B"), 0x41 => println!("Prefix: BIT 0, C"), 0x42 => println!("Prefix: BIT 0, D"), 0x43 => println!("Prefix: BIT 0, E"), 0x44 => println!("Prefix: BIT 0, H"), 0x45 => println!("Prefix: BIT 0, L"), 0x46 => println!("Prefix: BIT 0, (HL)"), 0x47 => println!("Prefix: BIT 0, A"), 0x48 => println!("Prefix: BIT 1, B"), 0x49 => println!("Prefix: BIT 1, C"), 0x4a => println!("Prefix: BIT 1, D"), 0x4b => println!("Prefix: BIT 1, E"), 0x4c => println!("Prefix: BIT 1, H"), 0x4d => println!("Prefix: BIT 1, L"), 0x4e => println!("Prefix: BIT 1, (HL)"), 0x4f => println!("Prefix: BIT 1, A"),
            0x50 => println!("Prefix: BIT 2, B"), 0x51 => println!("Prefix: BIT 2, C"), 0x52 => println!("Prefix: BIT 2, D"), 0x53 => println!("Prefix: BIT 2, E"), 0x54 => println!("Prefix: BIT 2, H"), 0x55 => println!("Prefix: BIT 2, L"), 0x56 => println!("Prefix: BIT 2, (HL)"), 0x57 => println!("Prefix: BIT 2, A"), 0x58 => println!("Prefix: BIT 3, B"), 0x59 => println!("Prefix: BIT 3, C"), 0x5a => println!("Prefix: BIT 3, D"), 0x5b => println!("Prefix: BIT 3, E"), 0x5c => println!("Prefix: BIT 3, H"), 0x5d => println!("Prefix: BIT 3, L"), 0x5e => println!("Prefix: BIT 3, (HL)"), 0x5f => println!("Prefix: BIT 3, A"),
            0x60 => println!("Prefix: BIT 4, B"), 0x61 => println!("Prefix: BIT 4, C"), 0x62 => println!("Prefix: BIT 4, D"), 0x63 => println!("Prefix: BIT 4, E"), 0x64 => println!("Prefix: BIT 4, H"), 0x65 => println!("Prefix: BIT 4, L"), 0x66 => println!("Prefix: BIT 4, (HL)"), 0x67 => println!("Prefix: BIT 4, A"), 0x68 => println!("Prefix: BIT 5, B"), 0x69 => println!("Prefix: BIT 5, C"), 0x6a => println!("Prefix: BIT 5, D"), 0x6b => println!("Prefix: BIT 5, E"), 0x6c => println!("Prefix: BIT 5, H"), 0x6d => println!("Prefix: BIT 5, L"), 0x6e => println!("Prefix: BIT 5, (HL)"), 0x6f => println!("Prefix: BIT 5, A"),
            0x70 => println!("Prefix: BIT 6, B"), 0x71 => println!("Prefix: BIT 6, C"), 0x72 => println!("Prefix: BIT 6, D"), 0x73 => println!("Prefix: BIT 6, E"), 0x74 => println!("Prefix: BIT 6, H"), 0x75 => println!("Prefix: BIT 6, L"), 0x76 => println!("Prefix: BIT 6, (HL)"), 0x77 => println!("Prefix: BIT 6, A"), 0x78 => println!("Prefix: BIT 7, B"), 0x79 => println!("Prefix: BIT 7, C"), 0x7a => println!("Prefix: BIT 7, D"), 0x7b => println!("Prefix: BIT 7, E"), 0x7c => println!("Prefix: BIT 7, H"), 0x7d => println!("Prefix: BIT 7, L"), 0x7e => println!("Prefix: BIT 7, (HL)"), 0x7f => println!("Prefix: BIT 7, A"),
            0x80 => println!("Prefix: RES 0, B"), 0x81 => println!("Prefix: RES 0, C"), 0x82 => println!("Prefix: RES 0, D"), 0x83 => println!("Prefix: RES 0, E"), 0x84 => println!("Prefix: RES 0, H"), 0x85 => println!("Prefix: RES 0, L"), 0x86 => println!("Prefix: RES 0, (HL)"), 0x87 => println!("Prefix: RES 0, A"), 0x88 => println!("Prefix: RES 1, B"), 0x89 => println!("Prefix: RES 1, C"), 0x8a => println!("Prefix: RES 1, D"), 0x8b => println!("Prefix: RES 1, E"), 0x8c => println!("Prefix: RES 1, H"), 0x8d => println!("Prefix: RES 1, L"), 0x8e => println!("Prefix: RES 1, (HL)"), 0x8f => println!("Prefix: RES 1, A"),
            0x90 => println!("Prefix: RES 2, B"), 0x91 => println!("Prefix: RES 2, C"), 0x92 => println!("Prefix: RES 2, D"), 0x93 => println!("Prefix: RES 2, E"), 0x94 => println!("Prefix: RES 2, H"), 0x95 => println!("Prefix: RES 2, L"), 0x96 => println!("Prefix: RES 2, (HL)"), 0x97 => println!("Prefix: RES 2, A"), 0x98 => println!("Prefix: RES 3, B"), 0x99 => println!("Prefix: RES 3, C"), 0x9a => println!("Prefix: RES 3, D"), 0x9b => println!("Prefix: RES 3, E"), 0x9c => println!("Prefix: RES 3, H"), 0x9d => println!("Prefix: RES 3, L"), 0x9e => println!("Prefix: RES 3, (HL)"), 0x9f => println!("Prefix: RES 3, A"),
            0xa0 => println!("Prefix: RES 4, B"), 0xa1 => println!("Prefix: RES 4, C"), 0xa2 => println!("Prefix: RES 4, D"), 0xa3 => println!("Prefix: RES 4, E"), 0xa4 => println!("Prefix: RES 4, H"), 0xa5 => println!("Prefix: RES 4, L"), 0xa6 => println!("Prefix: RES 4, (HL)"), 0xa7 => println!("Prefix: RES 4, A"), 0xa8 => println!("Prefix: RES 5, B"), 0xa9 => println!("Prefix: RES 5, C"), 0xaa => println!("Prefix: RES 5, D"), 0xab => println!("Prefix: RES 5, E"), 0xac => println!("Prefix: RES 5, H"), 0xad => println!("Prefix: RES 5, L"), 0xae => println!("Prefix: RES 5, (HL)"), 0xaf => println!("Prefix: RES 5, A"),
            0xb0 => println!("Prefix: RES 6, B"), 0xb1 => println!("Prefix: RES 6, C"), 0xb2 => println!("Prefix: RES 6, D"), 0xb3 => println!("Prefix: RES 6, E"), 0xb4 => println!("Prefix: RES 6, H"), 0xb5 => println!("Prefix: RES 6, L"), 0xb6 => println!("Prefix: RES 6, (HL)"), 0xb7 => println!("Prefix: RES 6, A"), 0xb8 => println!("Prefix: RES 7, B"), 0xb9 => println!("Prefix: RES 7, C"), 0xba => println!("Prefix: RES 7, D"), 0xbb => println!("Prefix: RES 7, E"), 0xbc => println!("Prefix: RES 7, H"), 0xbd => println!("Prefix: RES 7, L"), 0xbe => println!("Prefix: RES 7, (HL)"), 0xbf => println!("Prefix: RES 7, A"),
            0xc0 => println!("Prefix: SET 0, B"), 0xc1 => println!("Prefix: SET 0, C"), 0xc2 => println!("Prefix: SET 0, D"), 0xc3 => println!("Prefix: SET 0, E"), 0xc4 => println!("Prefix: SET 0, H"), 0xc5 => println!("Prefix: SET 0, L"), 0xc6 => println!("Prefix: SET 0, (HL)"), 0xc7 => println!("Prefix: SET 0, A"), 0xc8 => println!("Prefix: SET 1, B"), 0xc9 => println!("Prefix: SET 1, C"), 0xca => println!("Prefix: SET 1, D"), 0xcb => println!("Prefix: SET 1, E"), 0xcc => println!("Prefix: SET 1, H"), 0xcd => println!("Prefix: SET 1, L"), 0xce => println!("Prefix: SET 1, (HL)"), 0xcf => println!("Prefix: SET 1, A"),
            0xd0 => println!("Prefix: SET 2, B"), 0xd1 => println!("Prefix: SET 2, C"), 0xd2 => println!("Prefix: SET 2, D"), 0xd3 => println!("Prefix: SET 2, E"), 0xd4 => println!("Prefix: SET 2, H"), 0xd5 => println!("Prefix: SET 2, L"), 0xd6 => println!("Prefix: SET 2, (HL)"), 0xd7 => println!("Prefix: SET 2, A"), 0xd8 => println!("Prefix: SET 3, B"), 0xd9 => println!("Prefix: SET 3, C"), 0xda => println!("Prefix: SET 3, D"), 0xdb => println!("Prefix: SET 3, E"), 0xdc => println!("Prefix: SET 3, H"), 0xdd => println!("Prefix: SET 3, L"), 0xde => println!("Prefix: SET 3, (HL)"), 0xdf => println!("Prefix: SET 3, A"),
            0xe0 => println!("Prefix: SET 4, B"), 0xe1 => println!("Prefix: SET 4, C"), 0xe2 => println!("Prefix: SET 4, D"), 0xe3 => println!("Prefix: SET 4, E"), 0xe4 => println!("Prefix: SET 4, H"), 0xe5 => println!("Prefix: SET 4, L"), 0xe6 => println!("Prefix: SET 4, (HL)"), 0xe7 => println!("Prefix: SET 4, A"), 0xe8 => println!("Prefix: SET 5, B"), 0xe9 => println!("Prefix: SET 5, C"), 0xea => println!("Prefix: SET 5, D"), 0xeb => println!("Prefix: SET 5, E"), 0xec => println!("Prefix: SET 5, H"), 0xed => println!("Prefix: SET 5, L"), 0xee => println!("Prefix: SET 5, (HL)"), 0xef => println!("Prefix: SET 5, A"),
            0xf0 => println!("Prefix: SET 6, B"), 0xf1 => println!("Prefix: SET 6, C"), 0xf2 => println!("Prefix: SET 6, D"), 0xf3 => println!("Prefix: SET 6, E"), 0xf4 => println!("Prefix: SET 6, H"), 0xf5 => println!("Prefix: SET 6, L"), 0xf6 => println!("Prefix: SET 6, (HL)"), 0xf7 => println!("Prefix: SET 6, A"), 0xf8 => println!("Prefix: SET 7, B"), 0xf9 => println!("Prefix: SET 7, C"), 0xfa => println!("Prefix: SET 7, D"), 0xfb => println!("Prefix: SET 7, E"), 0xfc => println!("Prefix: SET 7, H"), 0xfd => println!("Prefix: SET 7, L"), 0xfe => println!("Prefix: SET 7, (HL)"), 0xff => println!("Prefix: SET 7, A"),
            _ => {}
        }
    }

    pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
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

    pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
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

            0xE0 => Some(Instruction::LDH(LoadType::Byte(LoadByteTarget::Address8, LoadByteSource::A))),
            0xE1 => Some(Instruction::POP(StackTarget::HL)),
            0xE2 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::AddressC, LoadByteSource::A))),
            0xE5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xE6 => Some(Instruction::AND(ArithmeticTarget::D8)),
            0xE7 => Some(Instruction::RST(RstTarget::Rst20H)),
            0xE8 => Some(Instruction::ADDSP(ArithmeticTarget::D8)),
            0xE9 => Some(Instruction::JP(JumpTest::Always, JumpCondition::AddressHL)),
            0xEA => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::Address16, LoadByteSource::A))),
            0xEE => Some(Instruction::XOR(ArithmeticTarget::D8)),
            0xEF => Some(Instruction::RST(RstTarget::Rst28H)),

            0xF0 => Some(Instruction::LDH(LoadType::Byte(LoadByteTarget::A, LoadByteSource::Address8))),
            0xF1 => Some(Instruction::POP(StackTarget::AF)),
            0xF2 => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::AddressC))),
            0xF3 => Some(Instruction::DI),
            0xF5 => Some(Instruction::PUSH(StackTarget::AF)),
            0xF6 => Some(Instruction::OR(ArithmeticTarget::D8)),
            0xF7 => Some(Instruction::RST(RstTarget::Rst30H)),
            0xF8 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL, LoadWordSource::SPR8))),
            0xF9 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP, LoadWordSource::HL))),
            0xFA => Some(Instruction::LD(LoadType::Byte(LoadByteTarget::A, LoadByteSource::Address16))),
            0xFB => Some(Instruction::EI),
            0xFE => Some(Instruction::CP(ArithmeticTarget::D8)),
            0xFF => Some(Instruction::RST(RstTarget::Rst38H)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
}
