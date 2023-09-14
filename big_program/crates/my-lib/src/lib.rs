use std::path::Path;
use std::fs;
use log::debug;

pub mod error;
use error::Error;

pub struct Module {}

impl Module {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, std::io::Error> {
        debug!("Loading wasm file from {:?}", path.as_ref());
        let bytes = fs::read(path.as_ref()).map_err(|e| Error::FileNotReadable(path.as_ref().to_path_buf(), e.to_string()))?;
        Self::new(&bytes)
    }
    pub fn new(bytes: &[u8]) -> Result<Self,> {

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
