use ::std::fs;

// Cartridge Unit
pub struct CU {
    pub buffer: Vec<u8>,
}

impl CU {
    pub fn new(path: &str) -> Result<CU, ()> {
        let buffer = fs::read(path).expect("ERROR: Unable to locate ROM");

        Ok(CU { buffer })
    }
}
