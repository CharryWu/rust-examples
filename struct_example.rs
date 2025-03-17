/**
 * Rust does NOT have OOP features, but it has `struct` similar to C and Golang
 * Below is an example of struct field & method definitions in rust lang
 */
struct Person {
    first_name: String, // fields
    last_name: String,
    age: u32,
}

impl Person {
    /**
     * Constructor - any fn with arbitrary name `new` `from` can be ctor
     */
    fn new() -> Person {
        Person {
            first_name: "Default first name".to_string(),
            last_name: "Default last name".to_string(),
            age: 0,
        }
    }

    /**
     * Constructor with parameters
     */
    fn from(first_name: String, last_name: String, age: u32) -> Person {
        Person {
            first_name, // same variable arg can be used as field
            last_name,
            age,
        }
    }
    fn clone(other: &Person) -> Person {
        Person {
            first_name: other.first_name.clone(), // same variable arg can be used as field
            last_name: other.last_name.clone(),
            age: other.age.clone(),
        }
    }

    // associated (static) function
    fn associated_function() {
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
    fn change_age(&mut self, new_age: u32) {
        println!("Person::change_age before age={}", self.age);
        self.age = new_age;
        println!("Person::change_age after new age={}", self.age);
    }
}

pub fn struct_example_driver() {
    println!("---------- struct::struct_example_driver ----------");
    Person::associated_function();
    let default_person = Person::new();

    println!(
        "default Person first_name={}, last_name={}, age={}",
        default_person.first_name, default_person.last_name, default_person.age
    );

    let param_inited_person = Person::from(String::from("JAJAJA"), String::from("DDD"), 10);

    println!(
        "param inited Person first_name={}, last_name={}, age={}",
        param_inited_person.first_name, param_inited_person.last_name, param_inited_person.age
    );

    let original_person = Person::from(String::from("CLONE"), String::from("CLONE"), 1);
    let cloned_person = Person::clone(&original_person);

    println!(
        "cloned Person first_name={}, last_name={}, age={}",
        cloned_person.first_name, cloned_person.last_name, cloned_person.age
    );

    let mut person = Person {
        first_name: "Filip".to_string(),
        last_name: "Jerga".to_string(),
        age: 30,
    };

    println!(
        "init Person first_name = \"{}\", last_name = \"{}\", age={}",
        person.first_name, person.last_name, person.age
    );

    person.change_age(33); // `person` moved as `self` of change_age

    println!(
        "changed Person first_name = \"{}\", last_name = \"{}\", age={}",
        person.first_name, person.last_name, person.age
    );
}
