use std::arch::x86_64::_bittest;
use std::thread::sleep;
use std::time::Duration;
use crate::{CPU::registres::Registers, Memory::memory::MemoryBus};

use super::instructions::{ArithmeticTarget, RstTarget, Instruction, JumpTest, StackTarget, LoadByteSource, LoadType, LoadByteTarget, LoadWordSource, LoadWordTarget, JumpCondition};

pub struct CPU {
    pub(crate) registers: Registers,
    pub(crate) pc: u16,
    pub(crate) bus: MemoryBus,
    pub(crate) sp: u16,
    pub(crate) interrupt_master_enable: bool,
    pub(crate) halt: bool,
    pub(crate) ei: u8,
    pub(crate) di: u8,
    pub(crate) cycle:u8,
}

impl CPU {

    pub const CPU_FREQ: u32 =4_194_304;

    pub fn read_next_byte(&mut self) -> u8 {
        //self.pc += 1;
        let byte = self.bus.read_byte(self.pc+1);
        byte
    }

    pub fn read_next_word(&mut self) -> u16 {
        //self.pc += 2;
        let word = self.bus.read_word(self.pc+1);
        word
    }

    pub fn execute(&mut self, instruction: Instruction) -> u16{
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    },
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    },
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    },
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    },
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    },
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    },
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    },
                    ArithmeticTarget::AddressHL => {
                        let value = self.bus.read_byte(self.registers.get_hl());
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    },
                    ArithmeticTarget::D8 => {
                        let value = self.read_next_byte();
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    },
                    _=> panic!("value 3connu"),
                };

                match target {
                    ArithmeticTarget::D8 => self.pc+2,
                    _ => self.pc+1,
                }
            },

            Instruction::ADDHL(target) => self.execute_add_hl(target),

            Instruction::ADC(target) => self.execute_adc(target),

            Instruction::SUB(target) => {
                self.execute_sub(target)
            },

            Instruction::SBC(target) => {
                self.execute_sbc(target)
                
            },

            Instruction::AND(target) => {
                self.execute_and(target)
            },

            Instruction::OR(target) => {
                self.execute_or(target)
            },

            Instruction::XOR(target) => {
                self.execute_xor(target)
            },

            Instruction::CP(target) => {
                self.execute_cp(target)
            },

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
                        self.registers.set_bc(new_value);
                    }
                    ArithmeticTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = value.wrapping_add(1);
                        self.registers.set_de(new_value);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = value.wrapping_add(1);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::SP => {
                        let value = self.sp;
                        let new_value = value.wrapping_add(1);
                        self.sp = new_value;
                    }
                    _ => panic!("value 4connu")
                };
                self.pc+1
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
                        self.registers.set_bc(new_value);
                    }
                    ArithmeticTarget::DE => {
                        let value = self.registers.get_de();
                        let new_value = value.wrapping_sub(1);
                        self.registers.set_de(new_value);
                    }
                    ArithmeticTarget::HL => {
                        let value = self.registers.get_hl();
                        let new_value = value.wrapping_sub(1);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::SP => {
                        let value = self.sp;
                        let new_value = value.wrapping_sub(1);
                        self.sp = new_value;
                    }
                    _ => panic!("value 5connu")
                };
                self.pc+1
            },

            Instruction::CCF => {
                self.execute_ccf();
                self.pc+1
            },

            Instruction::SCF => {
                self.execute_scf();
                self.pc+1
            },

            Instruction::CPL => {
                self.execute_cpl();
                self.pc+1
            },

            Instruction::RRA => {
                self.execute_rra();
                self.pc+1
            },

            Instruction::RLA => {
                self.execute_rla();
                self.pc+1
            },

            Instruction::RRCA => {
                self.execute_rrca();
                self.pc+1
            },

            Instruction::RLCA => {
                self.execute_rlca();
                self.pc+1
            },

            Instruction::BIT(target, bit_num) => {
                self.execute_bit(target, bit_num);
                self.pc+2
            },

            Instruction::RESET(target, bit_num) => {
                self.execute_reset(target, bit_num);
                self.pc+2
            },

            Instruction::SET(target, bit_num) => {
                self.execute_set(target, bit_num);
                self.pc+2
            },

            Instruction::SRL(target) => {
                self.execute_srl(target);
                self.pc+2
            },

            Instruction::RR(target) => {
                self.execute_rr(target);
                self.pc+2
            },

            Instruction::RL(target) => {
                self.execute_rl(target);
                self.pc+2
            },

            Instruction::RRC(target) => {
                self.execute_rrc(target);
                self.pc+2
            },

            Instruction::RLC(target) => {
                self.execute_rlc(target);
                self.pc+2
            },

            Instruction::SRA(target) => {
                self.execute_sra(target);
                self.pc+2
            },

            Instruction::SLA(target) => {
                self.execute_sla(target);
                self.pc+2
            },

            Instruction::SWAP(target) => {
                self.execute_swap(target);
                self.pc+2
            },

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
                            LoadByteSource::Address16 => {
                                let address = self.read_next_word();
                                self.bus.read_byte(address)
                            }
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
                            LoadByteTarget::Address16 => {
                                let address = self.read_next_word();
                                self.bus.write_byte(address,source_value);
                            } 
                            _ => { panic!("TODO: implement other targets") }
                        };
                        match (source, target) {
                            (LoadByteSource::D8, _)=> {
                                self.pc+2
                            }
                            (LoadByteSource::Address16, LoadByteTarget::A)
                            | (LoadByteSource::A, LoadByteTarget::Address16) => {
                                self.pc+3
                            }
                            _ => {
                                self.pc+1
                            }
                        }
                    }
                    LoadType::Word(target, source) => {

                        let source_value = match source {
                            LoadWordSource::D16 => {
                                let val = self.read_next_word();
                                self.pc+=2;
                                val
                            },
                            LoadWordSource::HL => self.registers.get_hl(),
                            LoadWordSource::SP => self.sp,
                            LoadWordSource::SPR8 => {
                                let r8 = (self.read_next_byte() as i8 as i32);
                                let sp = self.sp as i32;
                                let res = sp.wrapping_add(r8);

                                self.registers.f.zero=false;
                                self.registers.f.subtract=false;
                                self.registers.f.half_carry= (sp ^ r8 ^ res) & 0x10 != 0;
                                self.registers.f.carry= (sp ^ r8 ^ res) & 0x100 != 0;
                                res as u16
                            }
                        };
                        match target {
                            LoadWordTarget::BC => self.registers.set_bc(source_value),
                            LoadWordTarget::DE => self.registers.set_de(source_value),
                            LoadWordTarget::HL => self.registers.set_hl(source_value),
                            LoadWordTarget::SP => self.sp = source_value,
                            LoadWordTarget::Address16 => {
                                let address = self.read_next_word();
                                self.bus.write_word(address, source_value) },
                        };
                        match (source, target) {
                            (LoadWordSource::HL,LoadWordTarget::SP) => {
                                self.pc
                            },
                            (LoadWordSource::SPR8,LoadWordTarget::HL) => {
                                self.pc+2
                            }
                            _ => {
                                self.pc+1
                            }
                        }
                    }
                }
            }

            Instruction::LDH(load_type) => {

                match load_type {
                    LoadType::Byte(target, source) => {

                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::Address8 => {
                                let address = 0xFF00 | (self.read_next_byte() as u16);
                                
                                self.bus.read_byte(address)
                            },
                            _ => { panic!("TODO: implement other sources") }
                        };
                        //println!("{}",source_value);
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::Address8 => {
                                let address =  0xFF00 | self.read_next_byte() as u16;
                                self.bus.write_byte( address, source_value)
                            },
                            _ => { panic!("TODO: implement other targets") }
                        };
                        self.pc + 2
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
                };

                self.push(value);
                self.pc+1
            }

            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                    StackTarget::AF => {
                        self.registers.set_af(result);
                    },
                };
                self.pc+1
            }

            Instruction::CALL(test, target) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.call(jump_condition)
            }

            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.return_(jump_condition)
            }

            Instruction::DAA => {self.daa();self.pc+1}

            Instruction::JP(test, ju) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump(jump_condition,ju)
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
                self.pc
            }


            Instruction::RETI => {
                self.pc = self.pop();
                self.ei = 1;
                self.pc
            },

            Instruction::STOP => self.pc + 2,

            Instruction::HALT =>{self.halt = true;
                self.pc + 1},

            Instruction::EI => {
                //Like DI enabling take 1 more instructions to update IME flag
                self.ei=2;
                self.pc + 1
            },

            Instruction::DI => {
                //Like EI disabling take 1 more instructions to update IME flag
                self.di=2;
                self.pc + 1
            },

            Instruction::NOP => self.pc + 1,

            Instruction::RST(npc) => {
                let old_pc = self.pc+1;
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
                };
                self.pc
            },

            Instruction::ADDSP(targetd8) => self.execute_addsp(),
            Instruction::PrefixCB => panic!("value 6connu"),
        }
    }

    pub fn daa(&mut self) {
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

    pub fn add(&mut self, value: u8) -> u8 {
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

    pub fn execute_add_hl(&mut self, target: ArithmeticTarget) -> u16{
        let hl_value = self.registers.get_hl();
        let value = match target {
            ArithmeticTarget::BC => self.registers.get_bc(),
            ArithmeticTarget::DE => self.registers.get_de(),
            ArithmeticTarget::HL => self.registers.get_hl(),
            ArithmeticTarget::SP => self.sp,
            _ => panic!("value 7connu")

        };
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower 12 bits of the value and register HL
        // together result in a value bigger than 0xFFF. If the result is larger than 0xFFF,
        // then the addition caused a carry from the lower 12 bits to the upper 4 bits.
        self.registers.f.half_carry = (self.registers.get_hl() & 0xFFF) + (value & 0xFFF) > 0xFFF;
        self.registers.set_hl(new_value);
        self.pc+1
    }

    pub fn execute_addsp(&mut self) -> u16 {
        let signed_byte = self.read_next_byte() as u16;
        let value = self.sp;
        let (new_value, did_overflow) = self.sp.overflowing_add(signed_byte);
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower 12 bits of the value and register HL
        // together result in a value bigger than 0xFFF. If the result is larger than 0xFFF,
        // then the addition caused a carry from the lower 12 bits to the upper 4 bits.
        self.registers.f.half_carry = (self.registers.get_hl() & 0xFFF) + (value & 0xFFF) > 0xFFF;
        self.sp=new_value;
        self.pc + 2

    }

    pub fn execute_adc(&mut self, target: ArithmeticTarget) -> u16 {
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
            _=> panic!("value 8connu"),
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

        match target{
            ArithmeticTarget::D8 => self.pc + 2,
            _ => self.pc+1,
        }
    }

    pub fn execute_sub(&mut self, target: ArithmeticTarget) -> u16 {
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
            _ => panic!("value 9connu"),

        };
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F);

        self.registers.a = new_value;

        match target {
            ArithmeticTarget::D8 => self.pc+2,
            _ => self.pc+1,
        }
    }

    pub fn execute_sbc(&mut self, target: ArithmeticTarget) -> u16 {
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
            _ => panic!("value 10connu"),

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
        match target {
            ArithmeticTarget::D8 => self.pc+2,
            _ => self.pc+1,
        }
    }

    pub fn execute_and(&mut self, target: ArithmeticTarget) -> u16 {
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
            _ => panic!("value 11connu"),

        };
        let result = self.registers.a & value;

        self.registers.a = result;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true; // Half Carry est toujours défini sur true dans l'opération AND.
        self.registers.f.carry = false; // Carry est toujours défini sur false dans l'opération AND.

        match target {
            ArithmeticTarget::D8 => self.pc+2,
            _ => self.pc+1,
        }
    }

    pub fn execute_or(&mut self, target: ArithmeticTarget) -> u16 {
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
            _ => panic!("value 12connu"),

        };

        let result = self.registers.a | value;

        self.registers.a = result;

        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false; // Half Carry est toujours défini sur false dans l'opération OU.
        self.registers.f.carry = false; // Carry est toujours défini sur false dans l'opération OU.

        match target {
            ArithmeticTarget::D8 => self.pc+2,
            _ => self.pc+1,
        }
    }

    pub fn execute_xor(&mut self, target: ArithmeticTarget) -> u16 {
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
            _ => panic!("value 13connu"),

        };

        let result = self.registers.a ^ value;

        self.registers.a = result;
        
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false; // Half Carry est toujours défini sur false dans l'opération XOR.
        self.registers.f.carry = false; // Carry est toujours défini sur false dans l'opération XOR.
        match target {
            ArithmeticTarget::D8 => self.pc+2,
            _ => self.pc+1,
        }
    }

    pub fn execute_cp(&mut self, target: ArithmeticTarget) -> u16 {
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
            _ => panic!("value 14connu"),
            //AddressHL -> gethl read address dans memory
            //d8 readnextbyte
        };

        let (res, did_underflow) = self.registers.a.overflowing_sub(value);

        // Le résultat de la soustraction n'est pas stocké, seulement les drapeaux sont mis à jour.
        self.registers.f.zero = res==0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0x0F) < (value & 0x0F);
        //println!("a: {},value : {}",self.registers.a,value);
        self.registers.f.carry = did_underflow;
        match target {
            ArithmeticTarget::D8 => self.pc+2,
            _ => self.pc+1,
        }
    }

    pub fn execute_inc(&mut self, target: ArithmeticTarget) {
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
            _ => panic!("value 15connu")
        };

        // Mettez à jour les drapeaux appropriés.
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (new_value & 0x0F) == 0;
    }

    pub fn execute_dec(&mut self, target: ArithmeticTarget) {
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
            _ => panic!("value 16connu")
        };

        // Mettez à jour les drapeaux appropriés (Zéro, Soustraction, Demi-retenue).
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (new_value & 0x0F) == 0x0F;
    }

    pub fn execute_ccf(&mut self) {
        // Bascule la valeur du carry flag.
        self.registers.f.carry = !self.registers.f.carry;

        // Mettez à jour les autres drapeaux (subtract et half_carry) comme spécifié dans la documentation.
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    pub fn execute_scf(&mut self) {
        // Définir le drapeau de retenue (carry flag) sur vrai.
        self.registers.f.carry = true;

        // Réinitialiser les drapeaux N et H.
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = true;
    }

    pub fn execute_cpl(&mut self) {
        // Inversion de chaque bit du registre A.
        self.registers.a = !self.registers.a;

        // Réglage des drapeaux appropriés.
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
    }

    pub fn execute_rra(&mut self) {
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

    pub fn execute_rla(&mut self) {
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

    pub fn execute_rrca(&mut self) {
        let bit0 = self.registers.a & 0x01;

        self.registers.a >>= 1;

        // Le bit retenu est maintenant le bit0.
        self.registers.f.carry = bit0 != 0;

        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    pub fn execute_rlca(&mut self) {
        let bit7 = self.registers.a  & 0x80 > 0;

        self.registers.a <<= 1;

        // Le bit retenu est maintenant le bit7.
        self.registers.f.carry = bit7;

        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
    }

    pub fn execute_bit(&mut self, target: ArithmeticTarget, bit_num: u8) {
        let value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // You may need to implement memory read here.
            },
            _ => panic!("value 17connu"),
        };

        // Test the specified bit and set flags accordingly.
        let mask = 1 << bit_num;
        let bit_set = (value & mask) != 0;
        self.registers.f.zero = !bit_set;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
    }

    pub fn execute_reset(&mut self, target: ArithmeticTarget, bit_num: u8) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => panic!("value 18connu,{:?}",target),
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
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => panic!("value 19connu"),
        }
    }

    pub fn execute_set(&mut self, target: ArithmeticTarget, bit_num: u8) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => panic!("value 20connu"),
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
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => panic!("value 21connu"),
        }
    }

    pub fn execute_srl(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory
            },
            _ => panic!("value 22connu"),
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
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => panic!("value 23connu")
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    pub fn execute_rr(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => panic!("value 24connu"),
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
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => panic!("value 2connu"),
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    pub fn execute_rl(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => panic!("value 27connu"),
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
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => panic!("value 28connu"),
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    pub fn execute_rrc(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => panic!("value 29connu"),
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
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => panic!("value 30connu"),
        }

        // Update the flags.
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    pub fn execute_rlc(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => panic!("value 31connu"),
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
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => panic!("value 32connu"),
        }

        // Update the flags.
        self.registers.f.zero = false;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    pub fn execute_sra(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => panic!("value 33connu"),
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
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => panic!("value 3connu"),
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    pub fn execute_sla(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => panic!("value 37connu"),
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
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => panic!("value 38connu"),
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry;
    }

    pub fn execute_swap(&mut self, target: ArithmeticTarget) {
        let mut value = match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.read_byte(address) // Read the value from memory.
            },
            _ => panic!("value 1connu"),
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
            ArithmeticTarget::AddressHL => {
                let address = self.registers.get_hl();
                self.bus.write_byte(address, value); // Write the new value back to memory.
            },
            _ => panic!("value 2connu"),
        }

        // Update the flags.
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false; // Clear the carry flag.
    }

    pub fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);

    }

    pub fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        //println!("Poped : {}",(msb << 8) | lsb);
        (msb << 8) | lsb
    }

    pub fn run(&mut self) -> u8{
        self.update_ime();

        let interrupt = self.stat_interruption();
        if interrupt > 0 {
            self.cycle=4;
        } else {
            if self.halt {
                self.cycle=1; // noop
            } else {
                self.step();
            }
        }
        self.bus.run(self.cycle);
        self.cycle
    }


    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        let (instruction,cycle) = Instruction::from_byte(instruction_byte, prefixed);
        self.cycle=cycle;
        let next_pc = if let Some(instruction) = instruction {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };
        self.pc = next_pc;


    }

    pub fn jump(&mut self, should_jump: bool, ju : JumpCondition) -> u16 {
        match ju {

            JumpCondition::Address16 => {
                if should_jump {
                    let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
                    let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
                    self.cycle+=1;
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

    pub fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.cycle+=3;
            self.read_next_word()
        } else {
            next_pc
        }
    }

    pub fn jump_relative(&mut self, condition: bool) {
        if condition {
            self.cycle+=1;
            let r8=self.read_next_byte() as i8;
            let new_pc = ((self.pc as i32) + 2 + r8 as i32) as u16;
            self.pc = new_pc;
        } else {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    pub fn jump_indirect(&mut self) {
        let address = self.registers.get_hl(); // Obtenir la valeur de HL (adresse à sauter)
        self.pc = address; // Copier l'adresse dans le PC
    }

    pub fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.cycle+=3;
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn update_ime(&mut self) {
        if self.di > 0 {
            self.di -= 1;
            if self.di == 0 {
                self.interrupt_master_enable = false;
            }
        }

        if self.ei > 0 {
            self.ei -= 1;
            if self.ei == 0 {
                self.interrupt_master_enable = true;
            }
        }
    }

    fn stat_interruption(&mut self) -> u16 {

        //No interruption enable
        if !self.interrupt_master_enable && !self.halt {
            return 0;
        }

        //Flag on bus which is called https://gbdev.io/pandocs/Interrupts.html#ffff--ie-interrupt-enable
        let interruption = self.bus.interrupt_flags & self.bus.interrupt_enabled; //Operation on binary to get the right flag

        if interruption == 0 {
            return 0;
        }

        //Halt case return
        self.halt = false;
        if !self.interrupt_master_enable {
            return 0;
        }
        //Set to false cause will take one now
        self.interrupt_master_enable = false;
        let interrupt_jump_addresses: [u16; 5] = [0x40, 0x48, 0x50, 0x58, 0x60];
        for (flag_number, interrupt_jump_address) in interrupt_jump_addresses.iter().enumerate() {
            let flag = 1 << (flag_number as u8);
            if interruption & flag > 0 {
                //println!("interrutpion : {:b} interrutpion : {:b}",interruption,flag);
                self.bus.reset_interrupt(flag);
                let old_pc = self.pc;
                self.push(old_pc);
                self.pc = *interrupt_jump_address;
                return 4;
            }
        }
        panic!("Unknown interrupt was not handled! 0b{:08b}", interruption);
    }

}

