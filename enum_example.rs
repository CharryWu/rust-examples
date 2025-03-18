use std::fmt;

use crate::struct_example;

#[derive(Debug)]
pub enum PersonGenderEnum {
    Male,
    Female,
}
pub enum PersonIDEnum {
    Passport(String),
    IDCard(String),
}

impl fmt::Display for PersonGenderEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PersonGenderEnum::Male => write!(f, "PersonIDType::Passport"),
            PersonGenderEnum::Female => write!(f, "PersonIDType::IdentityCard"),
        }
    }
}
impl fmt::Display for PersonIDEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PersonIDEnum::Passport(_id_str) => write!(f, "PersonIDType::Passport:{}", _id_str),
            PersonIDEnum::IDCard(_id_str) => write!(f, "PersonIDType::IdentityCard:{}", _id_str),
        }
    }
}

pub struct ExtendedPerson {
    pub origin: struct_example::Person,
    pub gender: PersonGenderEnum,
    pub id: PersonIDEnum,
}

impl std::ops::Deref for ExtendedPerson {
    type Target = struct_example::Person;
    fn deref(&self) -> &Self::Target {
        &self.origin
    }
}

/**
 * Use `match` stmts to extract person id string
 */
pub fn extract_person_id(person: &ExtendedPerson) -> &String {
    match &person.id {
        PersonIDEnum::IDCard(id_str) => id_str,
        PersonIDEnum::Passport(id_str) => id_str,
    }
}

/**
 * Use `if let` stmts to extract person id string and compare to expected value
 */
pub fn check_person_id(person: &ExtendedPerson, expected: &String) -> bool {
    println!(
        "check_person_id, input person.id={}, expected target id={}",
        &person.id, &expected
    );
    if let PersonIDEnum::Passport(id_str) = &person.id {
        id_str == expected
    } else if let PersonIDEnum::IDCard(id_str) = &person.id {
        id_str == expected
    } else {
        false
    }
}
#[test]
pub fn test_enum() {
    println!("---------- enum_example::test_enum ----------");
    // See https://stackoverflow.com/a/32552688 for pattern of extending a struct
    let original_person = struct_example::Person::new();
    let extended_person_male = ExtendedPerson {
        origin: original_person,
        gender: PersonGenderEnum::Male,
        id: PersonIDEnum::Passport(String::from("EA5000000")),
    };
    println!(
        "extended_person_male.gender={:?}",
        extended_person_male.gender
    ); // use fmt::Debug trait
    println!(
        "extended_person_male.id={}",
        extract_person_id(&extended_person_male)
    ); // use match
    println!(
        "check_person_id female, res={}",
        check_person_id(&extended_person_male, &String::from("EA5000001"))
    ); // use if let
    println!(
        "extended_person_male first_name=\"{}\", last_name=\"{}\", age={}, gender={}, id=\"{}\"",
        extended_person_male.first_name,
        extended_person_male.last_name,
        extended_person_male.age,
        extended_person_male.gender,
        extended_person_male.id
    ); // use fmt::Display trait
    let original_person2 = struct_example::Person::new();
    let extended_person_female = ExtendedPerson {
        origin: original_person2,
        gender: PersonGenderEnum::Female,
        id: PersonIDEnum::IDCard(String::from("3400123456413")),
    };
    println!(
        "extended_person_female.id={}",
        extract_person_id(&extended_person_female)
    ); // use match
    println!(
        "extended Person female first_name=\"{}\", last_name=\"{}\", age={}, gender={}, id=\"{}\"",
        extended_person_female.first_name,
        extended_person_female.last_name,
        extended_person_female.age,
        extended_person_female.gender,
        extended_person_female.id
    ); // use fmt::Display trait
       // println!("{}", extended_person_female.id.clone()); // even PersonIDEnum::IDCard is associated with a string, it's still an Enum, not a `String`, cannot invoke `String` methods
    println!(
        "check_person_id female, res={}",
        check_person_id(&extended_person_female, &String::from("3400123456413"))
    ); // use if let
}
