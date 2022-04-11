use std::thread::spawn;

use Pointrs::pointer::Pointer;

fn main(){
    let x = 127;
    let y = Pointer(&x);
    spawn(move ||{
        assert!(*y == 127)
    }).join().unwrap();
}