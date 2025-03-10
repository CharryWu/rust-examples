pub fn dereference_str() {
    let mut message = String::from("Hello");
    let message2 = &mut message;
    message2.push_str(" World"); // automatic dereference, same as (*message2).push_str(" World");
    let message3 = &message2;
    // message3.push_str(" World"); // auto dereference will not work for double reference variables
}

pub fn dereference_number() {
    let a = 10;
    let b = &a;
    let c = &b;
    println!("{}", a == **c);
}

pub fn dereference_pointer() {
    let a = 10;
    let b = &a;
    let c = &b;
    let d = b;
    println!(
        "dereference_pointer: &a={:p}, b={:p}, c={:p}, d={:p}",
        &a, b, c, d
    );
}
