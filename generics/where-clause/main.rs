use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

//Because we would othersie ahve to expres this as T: Debug or
// use another method of indirect approach., this requires a where clause

impl<T> PrintInOption for T where
    Option<T>: Debug {
        //We want Option<T>: Debug as our bouind because that is what is 
        //being printed. Doing otherwise would be using the wrong bound
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

fn main() {
    let vec = vec![1,2,3];

    vec.print_in_option();
}

