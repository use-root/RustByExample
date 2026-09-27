#![allow(unused)]
#![allow(unused_must_use)]
struct Person<'a> {
    name:String,
    age:u32,
    friends_opinions:&'a str,
    feel_alone: bool
}

// Manual Debug

impl std::fmt::Debug for Person<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Person")
        .field("name", &self.name)
        .field("age", &self.age)
        .field("friends_opinions", &self.friends_opinions)
        .field("feel_alone", &self.feel_alone)
        .finish()
    }
}

// Change the way that you show your data 
impl std::fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User ({}, {}, {})", &self.age, &self.feel_alone, &self.friends_opinions)
    }
}


fn input_friends() -> String {
    let mut op_friends = String::new();
    let stdin = std::io::stdin();
    let resutl:std::io::Result<usize> = stdin.read_line(&mut op_friends);
    match resutl {
        // n = usize
        Ok(n) => println!(""),
        Err(error) => println!("Error {}", error),
    }
    op_friends
}


fn main () {
   let name_user = String::from("User");
  let binding = input_friends();
   let op_friends = &binding.trim();
   let user=Person {
    name:name_user,
     age:120,
      friends_opinions:op_friends,
       feel_alone: true,
   };
      println!("\n.............Display...........\n", );
      println!("\n{}\n", user); // Display
      println!("\n.............DEBUB...........\n", );
      println!("{:#?}",user); // Debug
      println!("{:?}",user); // Debug


    use std::fmt;
    use std::io::{self, Write};
    
    let mut some_writer = io::stdout();
    write!(&mut some_writer, "{}", format_args!("print with a {}", "macro"));
    
    fn my_fmt_fn(args: fmt::Arguments<'_>) {
        write!(&mut io::stdout(), "{args}");
    }
    my_fmt_fn(format_args!(", or a {} too\n", "function"));
}
