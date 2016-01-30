#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

//note the "-> '<-$labelname:" in the lables this differs to the similar concept in vba and c++ which is simply a "$labelname:"
//
//also note there is no "goto" you simply "break $labelname"
