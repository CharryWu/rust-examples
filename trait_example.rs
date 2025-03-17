pub trait Log {
    fn display_info(&self, prefix: &str);
    fn test_declared_fn(&self) {
        println!("Log::test_declared_fn");
    }
    fn test_associated_declared_fn() {
        println!("Log::test_associated_declared_fn");
    }
}
