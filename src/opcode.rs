pub struct Opcode {
    pub value: u16,
}

impl Opcode {
    pub fn from(value: u16) -> Self {
        Self { value }
    }

    pub fn _nnn(&self) -> u16 {
        self.value & 0x0FFF
    }

    pub fn __nn(&self) -> u8 {
        (self.value & 0x00FF) as u8
    }

    pub fn ___n(&self) -> u8 {
        (self.value & 0x000F) as u8
    }

    pub fn _x__(&self) -> u8 {
        ((self.value & 0x0F00) >> 8) as u8
    }

    pub fn __y_(&self) -> u8 {
        ((self.value & 0x00F0) >> 4) as u8
    }
}

pub enum Operation {
    ClearScreen,
    Jump(u16),
    SetRegister(u8, u8),
    AddRegister(u8, u8),
    SetIndexRegister(u16),
    Draw(u8, u8, u8),
}

impl Operation {
    pub fn from(opcode: Opcode) -> Self {
        match opcode.value {
            0x00e0 => Self::ClearScreen,
            0x1000..=0x1FFF => Self::Jump(opcode._nnn()),
            0x6000..=0x6FFF => Self::SetRegister(opcode._x__(), opcode.__nn()),
            0x7000..=0x7FFF => Self::AddRegister(opcode._x__(), opcode.__nn()),
            0xA000..=0xAFFF => Self::SetIndexRegister(opcode._nnn()),
            0xD000..=0xDFFF => Self::Draw(opcode._x__(), opcode.__y_(), opcode.___n()),
            _ => panic!("Unknown opcode: {:#X}", opcode.value),
        }
    }
}
