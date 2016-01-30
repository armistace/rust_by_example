//main.rs
//
fn used_function() {}

// allow dead code is and attribute that will allow dead code
//

#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}
//^ fix the compiler warning
//

fn main() {
    used_function();
}

