use crate::trait_example::{Log, ReflectLog};

use std::{
    any::Any,
    fmt::{self, Debug},
};
/**
 * Rust does NOT have OOP features, but it has `struct` similar to C and Golang
 * Below is an example of struct field & method definitions in rust lang
 */
pub struct Person {
    pub first_name: String, // fields
    pub last_name: String,
    pub age: u32,
}

impl Person {
    /**
     * Constructor - any fn with arbitrary name `new` `from` can be ctor
     */
    pub fn new() -> Person {
        Person {
            first_name: "Default first name".to_string(),
            last_name: "Default last name".to_string(),
            age: 0,
        }
    }

    /**
     * Constructor with parameters
     */
    pub fn from(first_name: String, last_name: String, age: u32) -> Person {
        Person {
            first_name, // same variable arg can be used as field
            last_name,
            age,
        }
    }
    pub fn clone(other: &Person) -> Person {
        Person {
            first_name: other.first_name.clone(), // same variable arg can be used as field
            last_name: other.last_name.clone(),
            age: other.age.clone(),
        }
    }

    // associated (static) function
    pub fn associated_function() {
        println!("Person::associated_function invoked!");
    }

    /**
     * method - invoked on a Person instance `&self`
     * NOTE: use `&self` instead of `s elf`, otherwise the instance of Person
     * will be MOVED INTO change_age method and DESTROYED when it goes out of
     * scope of `change_age`
     * In most cases use `&self` instead of `self`.
     *
     * Within an impl block, the type Self is an alias for the current type `Person`
     * You can also declare `change_age` as `fn change_age(self: &Self)`.
     */
    pub fn change_age(&mut self, new_age: u32) {
        println!("Person::change_age before age={}", self.age);
        self.age = new_age;
        println!("Person::change_age after new age={}", self.age);
    }
}

impl Log for Person {
    fn display_info(&self, prefix: &str) {
        if prefix.trim() == "" {
            println!(
                "\"{} {}\", age = {}",
                self.first_name, self.last_name, self.age
            );
        }
        println!(
            "{} \"{} {}\", age = {}",
            prefix, self.first_name, self.last_name, self.age
        );
    }

    /**
     * Override default impl of Log::test_declared_fn
     */
    fn test_declared_fn(&self) {
        println!("Person::test_declared_fn");
    }

    /**
     * Override default impl of Log::test_associated_declared_fn
     */
    fn test_associated_declared_fn() {
        println!("Person::test_associated_declared_fn");
    }
}

impl ReflectLog for Person {
    fn test_reflect_log(&self) {
        println!("Person::test_reflect_log");
    }
}

#[derive(Debug)]
pub struct Animal(pub String, pub u32, pub String); // create struct without any fields (tuple)
                                                    // NOTE: define field as `pub`. See https://stackoverflow.com/questions/24110970/tuple-struct-constructor-complains-about-private-fields
impl Log for Animal {
    fn display_info(&self, prefix: &str) {
        if prefix.trim() == "" {
            println!("type={}, age={} name={}", self.0, self.1, self.2);
        }
        println!("{} type={}, age={} name={}", prefix, self.0, self.1, self.2);
    }

    /**
     * Override default impl of Log::test_declared_fn
     */
    fn test_declared_fn(&self) {
        println!("Animal::test_declared_fn");
    }

    /**
     * Override default impl of Log::test_associated_declared_fn
     */
    fn test_associated_declared_fn() {
        println!("Animal::test_associated_declared_fn");
    }
}

impl ReflectLog for Animal {
    fn test_reflect_log(&self) {
        println!("Animal::test_reflect_log");
    }
}

pub fn struct_example_driver() {
    println!("---------- struct::struct_example_driver ----------");
    Person::associated_function();
    let default_person = Person::new();
    default_person.display_info("default Person");

    let param_inited_person = Person::from(String::from("JAJAJA"), String::from("DDD"), 10);

    param_inited_person.display_info("param inited Person");

    let original_person = Person::from(String::from("CLONE"), String::from("CLONE"), 1);
    let cloned_person = Person::clone(&original_person);
    cloned_person.display_info("cloned Person");

    let mut person = Person {
        first_name: "Filip".to_string(),
        last_name: "Jerga".to_string(),
        age: 30,
    };
    person.display_info("init Person");

    person.change_age(33); // `person` moved as `self` of change_age
    person.display_info("changed Person");

    let animal = Animal("Dog".to_string(), 10, "bulldog".to_string());
    animal.display_info("init animal");
    println!("init animal {:?}", animal);
    let Animal(animal_type, _, _) = animal;
    println!("extracted animal type {:?}", animal_type);
    // Log::test_associated_declared_fn(); // must call associated trait on implemented type
    Person::test_associated_declared_fn();
    Animal::test_associated_declared_fn();
}
