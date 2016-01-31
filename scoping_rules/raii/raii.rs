//raii.rs
fn create_box() {
    //allocate an integer on the heap
    let box1 = Box::new(3i32);
     //box1 is destroyed here and memory gets freed

}

fn main () {
    //Allocate an intger on the heap
    let box2 = Box::new(5i32);

    //A nested scope
    {
        let box3 = Box::new(4i32);

        //and here is where box3 gets destroyed!
    }

    //Create lot of boses just for fun
    //there is no need to manually free memory

    for _ in 0u32..1_000 {
        create_box();
    }

    //box 2 is destroyed and memory gets freed
}



