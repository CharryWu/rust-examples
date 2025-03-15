pub fn all() {
    println!("---------- stack::all ----------");
    c();
    d();
    f();
}

fn a() {
    println!("Calling A");
    e();
}

fn b() {
    println!("Calling B");
}

fn c() {
    println!("Calling C");
}

fn d() {
    println!("Calling D");
    a();
}

fn e() {
    println!("Calling E");
}

fn f() {
    println!("Calling F");
    b();
}

// xxd -g1 main
