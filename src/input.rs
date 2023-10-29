use std::sync::mpsc;



pub struct Input {
    up: Key,
    down: Key,
    left: Key,
    right: Key,
    a: Key,
    b: Key,
    select: Key,
    start: Key,
    pub(crate) input_reg: u8,
    pub interrupt: u8,
    key_data_receiver: mpsc::Receiver<Key>,
}

pub struct Key {
    pub key_type: KeyType,
    pub is_down: bool,
}

#[derive(Debug)]
pub enum KeyType {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

macro_rules! key {
    ($key_type:ident) => {
        Key {
            key_type: KeyType::$key_type,
            is_down: false,
        }
    };
}
macro_rules!  handle_key_event {
    ($self:ident, $key_field:ident, $key_type:ident, $key_data:ident) => {
        if $self.$key_field.is_down == $key_data.is_down {
            false
        } else {
            $self.$key_field.is_down = $key_data.is_down;
            true
        }
    };
}
impl Input {
    pub fn new(key_data_receiver: mpsc::Receiver<Key>) -> Self {
        Self {
            right: key!(Right),
            left: key!(Left),
            up: key!(Up),
            down: key!(Down),
            a: key!(A),
            b: key!(B),
            select: key!(Select),
            start: key!(Start),
            input_reg: 0,
            interrupt: 0,
            key_data_receiver,
        }
    }

    pub fn read(&self) -> u8 {
        self.input_reg
    }

    pub fn write(&mut self, value: u8) {
        self.input_reg = value;
        self.update_input_reg();
    }

    pub fn run(&mut self) {
        if let Ok(key) = self.key_data_receiver.try_recv() {
            let changed = match key.key_type {
                KeyType::Up => handle_key_event!(self, up, Up, key),
                KeyType::Down => handle_key_event!(self, down, Down, key),
                KeyType::Left => handle_key_event!(self, left, Left, key),
                KeyType::Right => handle_key_event!(self, right, Right, key),
                KeyType::A => handle_key_event!(self, a, A, key),
                KeyType::B => handle_key_event!(self, b, B, key),
                KeyType::Select => handle_key_event!(self, select, Select, key),
                KeyType::Start => handle_key_event!(self, start, Start, key),
            };

            if changed {
                if key.is_down {
                    self.interrupt |= 0x10;
                }
                self.update_input_reg();
            }
        }
    }

    fn update_input_reg(&mut self) {
        // filter to bit 4 & 5 as they are the only valid input bits
        self.input_reg &= 0x30;

        if self.input_reg & 0x10 == 0 {
            // bit 4 is low, check R L U D keys
            let row_res: u8 = self
                .col_1_keys()
                .iter()
                .filter_map(|key| if key.is_down { None } else { Some(key.key_type.value()) })
                .fold(0, |acc, key_value| acc | key_value);

            self.input_reg |= row_res;
        }

        if self.input_reg & 0x20 == 0 {
            // bit 5 is low, check A B Se St keys
            let row_res: u8 = self
                .col_0_keys()
                .iter()
                .filter_map(|key| if key.is_down { None } else { Some(key.key_type.value()) })
                .fold(0, |acc, key_value| acc | key_value);

            self.input_reg |= row_res;

        }
    }

    fn col_0_keys(&self) -> [&Key; 4] {
        [&self.a, &self.b, &self.select, &self.start]
    }

    fn col_1_keys(&self) -> [&Key; 4] {
        [&self.right, &self.left, &self.up, &self.down]
    }
}

impl KeyType {
    pub fn value(&self) -> u8 {
        // Input values have been incorrectly reordered, this shouldn't work, but it does
        match *self {
            KeyType::Right | KeyType::A => 0x01,
            KeyType::Left | KeyType::B => 0x02,
            KeyType::Up | KeyType::Select => 0x04,
            KeyType::Down | KeyType::Start => 0x08,
        }
    }

}
