use std::{thread, rc::Rc, sync::{Arc, Mutex}};
mod primitive_section;

#[derive(Debug)]
enum Cereal {
    Barely,
    Millet,
    Rice,
    Rye
}

fn integer() {
    let a = 10;
    let b = Box::new(20);
    let c = Rc::new(Box::new(30));
    let d = Arc::new(Mutex::new(40));
    print!("d {:?}", d);
}

fn main() {
    println!("Hvello, world!");
    //greet_world();
    //useThread();
    integer();
    //letter();
}

