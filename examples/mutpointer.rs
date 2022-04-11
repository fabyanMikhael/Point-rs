use std::{thread::spawn, ops:: AddAssign};

use Pointrs::pointer::MutPointer;

fn main(){
    let mut x = 0;
    let mut y = MutPointer(&mut x);
    spawn(move ||{
        y.add_assign(10);

    }).join().unwrap();

    assert!(x == 10)
}