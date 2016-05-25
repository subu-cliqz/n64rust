const RAM_SIZE: usize = 4  * 1024 * 1024;
const PIF_ROM_SIZE: usize = 2048;


pub struct InterConnect {
	pif_rom: Vec<u8>,
	ram: Vec<u16>
}

impl InterConnect {
	pub fn new(pif_rom: Vec<u8>) -> InterConnect {
		InterConnect {
		ram: vec![0; RAM_SIZE],
		pif_rom: pif_rom
		}
	}
}
