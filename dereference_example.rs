#[test]
pub fn test_dereference_str() {
    println!("---------- dereference_example::test_dereference_str ----------");
    let mut message = String::from("Hello");
    let message2 = &mut message;
    message2.push_str(" World"); // automatic dereference, same as (*message2).push_str(" World");
    let message3 = &message2;
    // message3.push_str(" World"); // auto dereference will not work for double reference variables
}
#[test]
pub fn test_dereference_number() {
    println!("---------- dereference_example::test_dereference_number ----------");
    let a = 10;
    let b = &a;
    let c = &b;
    println!("{}", a == **c);
}
#[test]
pub fn test_dereference_pointer() {
    println!("---------- dereference_example::test_dereference_pointer ----------");
    let a = 10;
    let b = &a;
    let c = &b;
    let d = b;
    println!(
        "dereference_pointer: &a={:p}, b={:p}, c={:p}, d={:p}",
        &a, b, c, d
    );
}
#[test]
pub fn test_assign_direct_ref_val() {
    println!("---------- dereference_example::test_assign_direct_ref_val ----------");
    let mut double_ref = &&100;
    println!("Before assign value = {}", double_ref);
    double_ref = &&50;
    println!("After assign value = {}", double_ref);
    // double_ref // CANNOT return ref to value owned by a fn
}
#[test]
pub fn test_assign_double_ref_val() {
    println!("---------- dereference_example::test_assign_double_ref_val ----------");
    let a = 10;
    let b = &a;
    let mut c = &b;
    println!("Before assign c value = {}", c);
    println!("Before assign c addr = {:p}", c);
    let double_ref = &&100;
    c = double_ref;

    println!("double_ref addr = {:p}", double_ref);
    println!(
        "Address of original value 100 can be reached by dereference double_ref twice &(**double_ref) = {:p}",
        &(**double_ref)
    );
    println!("After assign c value = {}", c);
    println!("After assign c addr = {:p}", c);
}
