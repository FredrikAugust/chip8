use std::{fs::File, io::Read};

use crate::opcode::{Opcode, Operation};

const MEMORY_SIZE: usize = 1024 * 4;
const ROM_START_ADDRESS: usize = 0x200;

#[derive(Debug)]
pub struct Chip {
    running: bool,
    registers: [u8; 16],
    index_register: u16,
    program_counter: usize,
    stack: Vec<u16>,
    memory: [u8; MEMORY_SIZE],
    loaded_rom: Option<String>,
}

impl Chip {
    pub fn run_rom(&mut self, rom: &str) {
        self.running = true;
        self.loaded_rom = Some(rom.to_string());
        self.load_memory_from_rom_file();
        self.run()
    }

    fn run(&mut self) {
        loop {
            if !self.running {
                break;
            }

            self.tick()
        }
    }

    fn tick(&mut self) {
        if self.program_counter >= MEMORY_SIZE {
            self.running = false;
            return;
        }

        let opcode = Opcode::from(self.fetch_opcode());
        self.execute_operation(Operation::from(opcode));
    }

    fn fetch_opcode(&self) -> u16 {
        let opcode_head: u16 = (self.memory[self.program_counter] as u16) << 8;
        let opcode_tail: u16 = self.memory[self.program_counter + 1].into();

        let opcode = opcode_head | opcode_tail;

        println!("Fetched opcode: {:#X}", opcode);

        opcode
    }

    fn execute_operation(&mut self, operation: Operation) {
        match operation {
            Operation::ClearScreen => self.clear_screen(),
            Operation::Jump(address) => self.jump(address),
            Operation::SetRegister(register, value) => self.set_register(register, value),
            Operation::AddRegister(register, value) => self.add_register(register, value),
            Operation::SetIndexRegister(address) => self.set_index_register(address),
            Operation::Draw(x, y, height) => self.draw(x, y, height),
        }
    }

    fn next_instruction(&mut self) {
        self.program_counter += 2;
    }

    fn get_flag(&self) -> u8 {
        self.registers[0xF]
    }

    fn set_flag(&mut self, value: u8) {
        self.registers[0xF] = value;
    }

    fn load_memory_from_rom_file(&mut self) -> &mut Self {
        let mut file = File::open(self.loaded_rom.as_ref().expect("No ROM path provided"))
            .expect("Unable to open ROM file");

        file.read(&mut self.memory[ROM_START_ADDRESS..])
            .expect("Could not read ROM");

        self
    }

    fn reset(&mut self) -> &mut Self {
        self.running = true;
        self.registers = [0; 16];
        self.index_register = 0;
        self.stack.clear();
        self.memory = [0; MEMORY_SIZE];
        self.program_counter = ROM_START_ADDRESS;

        self.load_memory_from_rom_file();

        self
    }

    pub fn new() -> Self {
        Self {
            running: false,
            registers: [0; 16],
            index_register: 0,
            program_counter: ROM_START_ADDRESS,
            stack: Vec::new(),
            memory: [0; MEMORY_SIZE],
            loaded_rom: None,
        }
    }

    fn clear_screen(&mut self) {
        println!("Clearing screen");
        self.next_instruction();
    }

    fn jump(&mut self, address: u16) {
        println!("Jumping to address: {:#X}", address);
        self.program_counter = address as usize;
    }

    fn set_register(&mut self, register: u8, value: u8) {
        println!("Setting register {} to {}", register, value);
        self.registers[register as usize] = value;
        self.next_instruction();
    }

    fn add_register(&mut self, register: u8, value: u8) {
        println!("Adding {} to register {}", value, register);
        self.registers[register as usize] += value;
        self.next_instruction();
    }

    fn set_index_register(&mut self, value: u16) {
        println!("Setting index register to {}", value);
        self.index_register = value;
        self.next_instruction();
    }

    fn draw(&mut self, x: u8, y: u8, nibble: u8) {
        println!("{},{},{}", x, y, nibble);
        self.next_instruction();
    }
}
