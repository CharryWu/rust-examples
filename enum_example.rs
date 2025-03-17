use std::fmt;

use crate::struct_example;
#[derive(Debug)]
pub enum PersonIDType {
    Passport,
    IdentityCard,
}
impl fmt::Display for PersonIDType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PersonIDType::Passport => write!(f, "PersonIDType::Passport"),
            PersonIDType::IdentityCard => write!(f, "PersonIDType::IdentityCard"),
        }
    }
}

pub struct ExtendedPerson {
    pub origin: struct_example::Person,
    pub id_type: PersonIDType,
}

impl std::ops::Deref for ExtendedPerson {
    type Target = struct_example::Person;
    fn deref(&self) -> &Self::Target {
        &self.origin
    }
}

pub fn enum_example_driver() {
    println!("---------- enum_example::enum_example_driver ----------");
    // See https://stackoverflow.com/a/32552688 for pattern of extending a struct
    let original_person = struct_example::Person::new();
    let extended_person = ExtendedPerson {
        origin: original_person,
        id_type: PersonIDType::Passport,
    };
    println!(
        "extended Person first_name=\"{}\", last_name=\"{}\", age={}, id_type={}",
        extended_person.first_name,
        extended_person.last_name,
        extended_person.age,
        extended_person.id_type
    ); // use fmt::Display trait
    println!("extended_person.id_type={:?}", extended_person.id_type); // use fmt::Debug trait
}
