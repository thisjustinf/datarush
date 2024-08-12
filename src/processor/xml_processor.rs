trait Processor {
    fn process(&self) -> Result<(), Box<dyn std::error::Error>>;
}
