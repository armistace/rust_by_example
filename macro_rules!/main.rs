// This is a simple macro named 'say_hello'

macro_rules! say_hello {
    () => (
        println!("Hello");
        )
}

fn main () {
    //This call will expand into 'println!("Hello");
    say_hello!()
}
