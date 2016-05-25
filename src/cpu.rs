
const NUM_GPR:usize = 32;

#[derive(Default, Debug)]
pub struct Cpu {
	reg_gpr: [u64; NUM_GPR],
	reg_fpr: [f64; NUM_GPR],

	reg_pc: u64,

	reg_hi: u64,
	reg_lo: u64,

	reg_llbit: bool, //TODO: Enum ?

	reg_fcr0: u32,
	reg_fcr31: u32,
	cp0: Cp0
}

impl Cpu {

	pub fn power_on_reset(&mut self) {
		self.cp0.power_on_reset();
	}
}

#[derive(Debug)]
enum RegConfigEp {
	D,
	DxxDxx,
	RFU
}

impl Default for RegConfigEp {
	fn default() -> RegConfigEp{
		RegConfigEp::D
	}
}

#[derive(Debug)]
enum RegConfigBe {
	LittteEndian,
	BigEndian
}

impl Default for RegConfigBe {
	fn default() -> RegConfigBe{
		RegConfigBe::BigEndian
	}
}


#[derive(Default, Debug)]
struct RegConfig {
	reg_config_ep: RegConfigEp,
	reg_config_be: RegConfigBe
}

impl RegConfig {
	fn power_on_reset(&mut self) {
		self.reg_config_ep = RegConfigEp::D;
		self.reg_config_be = RegConfigBe::BigEndian;
	}
}

#[derive(Default, Debug)]
struct Cp0 {
	reg_config: RegConfig
}

impl Cp0 {

	fn power_on_reset(&mut self) {
		self.reg_config.power_on_reset();
	}
}

