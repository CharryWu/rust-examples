pub fn extend_age(mut a: u32) {
    a += 100; // only the parameter `a` is altered, the original argument variable from caller1 is NOT altered due to being copied
}
#[test]
pub fn test_fn_arg_copied() {
    println!("---------- copy_example::test_fn_arg_copied ----------");
    let age = 30;
    extend_age(age); // `age` is COPIED into extend_age's `a` parameter, even though `a` is declared as a mutable parameter
    println!("Copy example1: age = {}", age); // output 30, since `age` itself is not modified, only its copy `a` got modified
}
