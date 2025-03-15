pub fn str_slice_example() {
    println!("---------- slice_example::str_slice_example ----------");
    let mut message = String::from("Hello");
    let slice = &message[2..4]; // message[2, 4) == "ll"
    println!("{}", slice);
}
