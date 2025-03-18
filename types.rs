use crate::type_utils;
#[test]
pub fn test_basic_types() {
    println!("---------- types::test_basic_types ----------");
    let message = "Hello, world!";
    let num = 10;
    let small_num: u8 = 10; // unsinged int of 8 bits
                            // 0 ~ 255 = 2^8 - 1
                            // 266 need i16
    let small_num_signed: i8 = 127;
    let sys_num: usize = 10; // depends on system bit width, 32 or 64
    let custom_num = 98_000;
    let hex_num = 0xfa;
    let bin_num = 0b1010_1010;
    let byte_num = b'A';
    let int_heap = Box::new(333);

    println!(
        "message {} = {}",
        type_utils::get_type_of(&message),
        message
    );
    println!("num {} = {}", type_utils::get_type_of(&num), num);
    println!(
        "small_num {} = {}",
        type_utils::get_type_of(&small_num),
        small_num
    );
    println!(
        "small_num_signed {} = {}",
        type_utils::get_type_of(&small_num_signed),
        small_num_signed
    );
    println!(
        "sys_num {} = {}",
        type_utils::get_type_of(&sys_num),
        sys_num
    );
    println!(
        "custom_num {} = {}",
        type_utils::get_type_of(&custom_num),
        custom_num
    );
    println!(
        "hex_num {} = {}",
        type_utils::get_type_of(&hex_num),
        hex_num
    );
    println!(
        "bin_num {} = {}",
        type_utils::get_type_of(&bin_num),
        bin_num
    );
    println!(
        "byte_num {} = {}",
        type_utils::get_type_of(&byte_num),
        byte_num
    );
}
#[test]
pub fn test_compound_types() {
    println!("---------- types::test_compound_types ----------");
    let tuples = (1.3, "Hello World!", true);
    println!("{:?}", tuples);
    println!("{} {} {}", tuples.0, tuples.1, tuples.2);
    // tuple unpacking
    let (x, y, z) = tuples;
    println!("{} {} {}", x, y, z);
    // array
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    println!("{} {} {} {} {}", arr[0], arr[1], arr[2], arr[3], arr[4]);
    let repeated_arr = [1; 5];
    println!("Repeated array: {:?}", repeated_arr);
}
