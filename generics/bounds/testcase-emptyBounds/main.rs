struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

//These function are only valied for ytpes which implelemnt these
//traits. The Fact that the traits are empty is irrelevant
//

fn red<T: Red>(_: &T)   -> &'static str {"Red"}
fn blue<T: Blue>(_: &T) -> &'static str {"Blue"}

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    //red() won't work on a blue jat nor vice versa
    //because of bounds

    println!("A carindal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));

    //println!("A turdy is {}", red(&_turkey));
    //TODO Try uncommenting this line.
}
