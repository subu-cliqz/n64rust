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

	pub fn read_word(&self, addr: u32) -> u32{
		if addr >= 0x1fc0_0000 && addr < 0x1fc0_07c0 {
			let rel_addr = addr-0x1fc0_0000;
			//TODO: Check endianness, check byte order crate
			((self.pif_rom[rel_addr as usize] as u32) << 24) |
			((self.pif_rom[(rel_addr + 1) as usize] as u32) << 16) |
			((self.pif_rom[(rel_addr + 2) as usize] as u32) << 8) |
			((self.pif_rom[(rel_addr + 3) as usize] as u32))
		} else {
			panic!("Unrecognized address : {:#x}", addr);
		}
	}
}
