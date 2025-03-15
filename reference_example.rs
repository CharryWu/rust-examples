// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

pub fn ref_assign_basic() {
    println!("---------- reference_example::ref_assign_basic ----------");
    // String is a smart pointer that doesn't own the data
    // &String reference is a simple pointer that doesn't own the data
    let message = String::from("Hello");
    let message2: &String = &message; // created a reference to message, `message2` "borrows" the reference to `message`, instead of the original `message`
                                      // `message2` is not owner of underlying data on heap, it points to `message1`
    println!("Reference example: message = {}", message);
    println!("Reference example: &message2 = {}", message2);
    // `message2`::drop() is NOT called, because it doesn't have ownership of what it refers to, automatically reclaimed by stack space;
    // however `message`::drop() is still called, which deallocates underlying memory of string bytes from heap
}

/**
 * We cannot have a mutable reference while we have an immutable one to the same value.
 * Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used
 */
pub fn caller_mut_immut_reference() {
    println!("---------- reference_example::caller_mut_immut_reference ----------");
    let mut s = String::from("hello");
    // Users of an immutable reference don’t expect the value to suddenly change out from under them!
    let r1 = &s; // no problem
    let r2 = &s; // no problem, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
                 // let r3 = &mut s; // BIG PROBLEM, this will affect all immutable borrows above, so is not permitted by rust compiler

    println!("caller_mutable_immutable_reference: r1={r1}, r2={r2}");
    // println!("caller_mutable_immutable_reference: r3={r3}");
}

/**
 * Note that a reference’s scope starts from where it is introduced and continues
 * through the last time that reference is used.
 * For instance, this code will compile because the last usage of the immutable references is in the println!,
 * before the mutable reference is introduced:
*/
pub fn caller_reference_scope() {
    println!("---------- reference_example::caller_reference_scope ----------");
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}"); // scopes of the immutable references r1 and r2 end after the println! where they are last used
                               // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem, r3 scope doesn't overlap with r1 and r2 scope, so this is allowed
    println!("{r3}"); // no problem
}

////////////////////// MORE EXAMPLES //////////////////////
fn mut_str_append(val: &mut String) {
    val.push_str("_unpredictable");
}
pub fn caller_mut_immut_reference2() {
    println!("---------- reference_example::caller_mut_immut_reference2 ----------");
    let mut message = String::from("Hello");
    let message2: &mut String = &mut message; // mutable borrow
    message2.push_str(" World"); // message2 gets auto-dereferenced, underlying str data is accessed and mutated
    mut_str_append(message2);
    println!("Reference example2: message = {}", message);
    // unpredictable_mutate(message2); // cannot access mutable borrow after immutable borrow
}

pub fn caller_reference_scope2() {
    let mut message = String::from("Hello");
    let message2: &mut String = &mut message; // mutable borrow
    mut_str_append(message2);
    let message3 = &message;
}
