use crate::type_utils;
#[test]
pub fn test_str_slice_example() {
    println!("---------- slice_example::test_str_slice_example ----------");
    let message = String::from("Hello");
    let slice = &message[2..4]; // message[2, 4) == "ll"
    println!("slice = {}", slice);
    println!("slice type = {}", type_utils::get_type_of(&slice));
    println!("slice.len = {}", slice.len());
}

#[test]
pub fn test_str_slice_clone() {
    println!("---------- slice_example::test_str_slice_clone ----------");
    let mut message = String::from("Hello");
    let message_clone = message.clone(); // deep clone
    println!("before clear message = {}", message);
    println!("before clear message_clone = {}", message_clone);
    message.clear();
    println!("after clear message = {}", message);
    println!("after clear message_clone = {}", message_clone);
}
