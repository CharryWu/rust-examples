fn main() {
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

    println!("{}", message);
    println!("{}", num);
    println!("{}", small_num);
    println!("{}", small_num_signed);
    println!("{}", sys_num);
    println!("{}", custom_num);
    println!("{}", hex_num);
    println!("{}", bin_num);
    println!("{}", byte_num);
}
