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
