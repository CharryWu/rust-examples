pub fn extend_age(mut a: u32) {
    a += 100; // only the parameter `a` is altered, the original argument variable from caller1 is NOT altered due to being copied
}
pub fn caller1() {
    let age = 30;
    extend_age(age); // `age` is COPIED into extend_age's `a` parameter
    println!("Copy example1: age = {}", age); // output 30, since age is not modified, only its copy got modified
}
