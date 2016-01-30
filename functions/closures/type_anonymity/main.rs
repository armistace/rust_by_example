// 'F' must implement 'Fn for a cosure which takes no 
// inputs and returns nothing - exactly what is required
// for "print".

fn apply<F>(f: F) where
    F: Fn() {
    f()
}

fn main () {
    let x = 7;

    //Catprue x into an anon type and implement
    //Fn for it. Store it in print
    let print = || println!("{}", x);

    apply(print);
}

