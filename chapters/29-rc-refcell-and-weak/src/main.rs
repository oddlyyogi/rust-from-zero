use std::rc::Rc;

fn main() {
    let strong = Rc::new(5);
    let note = Rc::downgrade(&strong);
    println!("{:?}", note.upgrade());
    drop(strong);
    println!("{:?}", note.upgrade());
}
