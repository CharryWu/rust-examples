use crate::struct_example::{Animal, Person};
pub trait Log {
    fn display_info(&self, prefix: &str);
    fn test_declared_fn(&self) {
        println!("Log::test_declared_fn");
    }

    fn test_associated_declared_fn() {
        println!("Log::test_associated_declared_fn");
    }
}

pub trait ReflectLog {
    fn test_reflect_log(&self);
}

/**
 * Compiler determines which fn to call at compile time.
 * Gets replaced by Animal::reflect_log and Person::reflect_log after compilation
 * Create multiple versions of function, depending on how many struct types
 * implement Log trait (Animal/Person)
 */
pub fn reflect_log(val: impl Log) {
    val.test_declared_fn();
}

/**
 * dyn is short for dynamic dispatch, determine which function to call at RUNTIME
 * Smaller output file since less repetition of code
 */
pub fn reflect_log2(val: &dyn ReflectLog) {
    val.test_reflect_log();
}

pub fn trait_example_caller() {
    println!("---------- trait_example::trait_example_caller ----------");
    let person = Person::new();
    let animal = Animal("Cat".to_string(), 5, "Egyptian Mau".to_string());
    reflect_log2(&person);
    reflect_log2(&animal);
    reflect_log(person);
    reflect_log(animal);
}
