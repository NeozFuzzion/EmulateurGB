struct CPU { 
    registers: Registers 
}
enum Instruction {
    ADD(ArithmeticTarget),
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}
  
impl CPU {
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
                let value = self.registers.c;
                let new_value = self.add(value);
                self.registers.a = new_value;
            }
            ArithmeticTarget::E => {
                let value = self.registers.c;
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
            //TODO HL d8
        }
      }
      _ => { /* TODO: support more instructions */ }
    }
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
}

