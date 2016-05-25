use super::cpu::Cpu;
use super::interconnect;


pub struct N64 {
	cpu: Cpu

}

impl N64 {

	pub fn new(pif_rom: Vec<u8>) -> N64 {
		N64 {
		cpu: Cpu::new (interconnect::InterConnect::new(pif_rom))
		}
	}
	pub fn power_on_reset(&mut self) {
		self.cpu.power_on_reset();
	}

	pub fn run(&mut self) {
		self.cpu.run()
	}
}
