use std::path::Path;
use log::debug;

pub struct Module {}

impl Module {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, std::io::Error> {
        debug!("Loading wasm file from {:?}", path.as_ref());
        Ok(Self{})
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_wasm_file() {
        let result = Module::from_file("./tests/test.wasm");
        assert!(result.is_ok());
    }
}
