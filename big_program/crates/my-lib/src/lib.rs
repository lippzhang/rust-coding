use std::path::Path;

pub struct Module {}

impl Module {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, std::io::Error> {
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
