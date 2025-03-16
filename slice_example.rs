use crate::type_utils;
pub fn str_slice_example() {
    println!("---------- slice_example::str_slice_example ----------");
    let message = String::from("Hello");
    let slice = &message[2..4]; // message[2, 4) == "ll"
    println!("slice = {}", slice);
    println!("slice type = {}", type_utils::get_type_of(&slice));
    println!("slice.len = {}", slice.len());
}
