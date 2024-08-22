use crate::first::say_hello as say_hello_first;

pub fn say_hello(){
    println!("hello from second modules");
}

#[test]
fn test_say_hello_crate() {
    say_hello();
    say_hello_first();
}
