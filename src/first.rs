
use crate::third::say_hello as say_hello_third;
pub fn say_hello(){
    println!("hello from first modules");
    say_hello_third()
}

pub mod second {
    pub mod third{
        pub fn say_hello() {
            super::super::say_hello();
        }
    }
}