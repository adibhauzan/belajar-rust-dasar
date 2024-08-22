    pub struct User {
        first_name: String,
        middle_name: String,
        last_name: String,
        email: String,
        age: u8,
    }

    impl User{
        pub fn new(first_name: String, middle_name: String, last_name: String, email: String, age: u8)-> User{
            User {
                first_name,
                middle_name,
                last_name,
                email,
                age
            }
        }
        pub fn say_hello(&self, name: &str){
            println!("Hello {}, my name is {}", name, self.first_name)
        }
    }
