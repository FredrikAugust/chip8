use std::{fs::File, io::Read};

use crate::opcode::Opcode;

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

        let opcode = self.fetch_opcode();
        self.execute_opcode(opcode);
    }

    fn fetch_opcode(&self) -> u16 {
        let opcode_head: u16 = (self.memory[self.program_counter] as u16) << 8;
        let opcode_tail: u16 = self.memory[self.program_counter + 1].into();

        let opcode = opcode_head | opcode_tail;

        println!("Fetched opcode: {:#X}", opcode);

        opcode
    }

    fn execute_opcode(&mut self, opcode: u16) {
        let opcode = Opcode::from(opcode);

        match opcode.value {
            0x00e0 => {
                println!("[*] Clear the screen");
                self.next_instruction();
            }
            0x00ee => {
                println!("[*] Return from subroutine");
                let stack_top = self.stack.pop().unwrap();
                self.program_counter = stack_top as usize;
            }
            0x1000..=0x1FFF => {
                println!("[*] Jump to address: {:#X}", opcode._nnn());
                self.program_counter = opcode._nnn() as usize;
            }
            0x2000..=0x2FFF => {
                println!("[*] Call subroutine at address {:#X}", opcode._nnn());
                self.stack.push(self.program_counter as u16);
                self.program_counter = opcode._nnn() as usize;
            }
            0xb000..=0xbFFF => {
                println!("[*] Jump to address {:#X} + V0", opcode._nnn());
                self.program_counter = (opcode._nnn() + self.registers[0] as u16) as usize;
            }
            0xc000..=0xcfff => {
                println!(
                    "[*] Set register V{:X} to random number AND {:#X}",
                    opcode._x__(),
                    opcode.__nn()
                );
                self.registers[opcode._x__() as usize] = rand::random::<u8>() & (opcode.__nn());
                self.next_instruction();
            }
            _ => {
                println!("[?] Unknown opcode: {:#X}", opcode.value);
                self.next_instruction();
            }
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
}
