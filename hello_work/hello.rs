macro_rules! sayhello { 
    ($name:expr) => { 
        println!("Hola {}",$name)
      };
}

fn main(){
    sayhello!("Eze")
}
