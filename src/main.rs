mod cpu;
mod opcode;

pub use cpu::Chip;

fn main() {
    let mut chip = Chip::new();

    chip.run_rom("src/IBM Logo.ch8");

    println!("Hello, world!");
}
