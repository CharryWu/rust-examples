use std::env;
mod cat;
mod copy_example;
mod dereference_example;
mod enum_example;
mod hashmap_example;
mod move_example;
mod reference_example;
mod slice_example;
mod stack;
mod struct_example;
mod trait_example;
mod type_utils;
mod types;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    cat::kitten::meow();
}
