use std::error;

pub trait Storable {
    fn save(&self, filename: &str) -> Result<(), Box<dyn error::Error>>;
    fn load(&mut self, filename: &str) -> Result<(), Box<dyn error::Error>>;
}
