pub fn print_msg(msg: String) {
    println!("{}", msg);
    let msg2 = msg;
    // println!("{}", msg); // msg becomes invalid after move, so this line throws error
    println!("{}", msg2); // original `message` from caller1 is moved to `msg`, then moved to `msg2`
} // `msg` is going out of scope, but nothing more will happen, because it was moved
  // `msg2` is going out of scope, and its `drop` will be called which clears the underlying memory of string bytes from heap
#[test]
pub fn test_immu_moved_to_fn_arg() {
    println!("---------- move_example::test_immu_moved_to_fn_arg ----------");
    let message = String::from("test");
    print_msg(message);
    // println!("{}", message); // will throw error since `message` is moved into print_msg's argument `msg`
} // `message` is going out of scope because it was moved into print_msg

pub fn extend_msg(mut a: String) -> String {
    a.push_str(" World");
    a
}
#[test]
pub fn test_mut_reuse_not_moved() {
    println!("---------- move_example::test_mut_reuse_not_moved ----------");
    let mut message = String::from("Hello");
    message = extend_msg(message);
    println!("Move example: caller2 message = {}", message);
}
