//Module pour la lecture et l'écriture des entrées et des sorties
use std::io;
fn main() {
    let mut _name = String::new();
    println!("What's your name :");
    io::stdin().read_line(&mut _name).expect("Exception detect");
    println!("I'm {}", _name);
}
