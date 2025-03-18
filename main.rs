mod cat;
mod copy_example;
mod dereference_example;
mod enum_example;
mod hashmap_example;
mod move_example;
mod reference_example;
mod slice_example;
mod stack;
mod struct_example;
mod trait_example;
mod type_utils;
mod types;
fn main() {
    cat::kitten::meow();
}

use rust_examples::LibPerson;
// fn lib_runner() {
//     let lib_person = LibPerson {
//         full_name: "Craig Hellen".to_string(),
//         age: 33,
//     };

// }
