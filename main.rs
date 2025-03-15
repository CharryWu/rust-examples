mod cat;
mod copy_example;
mod dereference_example;
mod move_example;
mod reference_example;
mod stack;
fn main() {
    copy_example::caller1();
    move_example::caller2();
    reference_example::ref_assign_basic();
    reference_example::caller_mut_immut_reference();
    reference_example::caller_reference_scope();
    reference_example::caller_mut_immut_reference2();
    reference_example::caller_reference_scope2();
    dereference_example::dereference_str();
    dereference_example::dereference_number();
    dereference_example::dereference_pointer();
    dereference_example::assign_direct_ref_val();
    dereference_example::assign_double_ref_val();
    cat::kitten::meow();
    stack::all();
}
