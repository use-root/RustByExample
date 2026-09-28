//#[derive(Debug)]
// Tuple struct
//// An attribute to hide warnings for unused code.
#![allow(dead_code)]
struct PersonA(u16, u16, f32);

// Manual Implementation of debug a tuple struct

//impl std::fmt::Debug for Person {
 //  fn fmt(&self, f:&mut std::fmt::Formatter<'_>)  -> std::fmt::Result {
  //     f.debug_tuple("Person")
   //     .field(&self.0)
    //    .field(&self.1)
     //   .field(&self.2)
      //  .finish()
 //  }
//}


//impl std::fmt::Debug for Person {
 //   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  //      write!(f, "Person [Age: {}, Height: {}, Weight: {}]", self.0, self.1, self.2)
   // }
//}


impl std::fmt::Debug for PersonA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            // Custom logic for pretty-printing {:#?}
           //write!(f, "Person [Age: {}, Height: {}, Weight: {}]", self.0, self.1, self.2);
           write!(f, "Person {{\n\t{} \n\t{} \n\t{}\n}}", self.0, self.1, self.2)
        } else {
             //Logic for standard {:?}
            write!(f, "Person({}, {}, {})", self.0, self.1, self.2)
        }
    }
}


// We use compotition (has a..)

#[derive(Debug)]

struct Identification {
    name: String,
    number_id: String,
}

impl Identification{
    fn display_id(&self)  ->  &str{
        &self.number_id
    }
}

#[derive(Debug)]
struct Student {
    identification: Identification,
    age: u8,
}

impl Student {  
    fn is_adult(&self) -> bool { 
        self.age >=21
    }

    fn introduce(&self){
        println!("I'm nice to meet you! I'm {}", self.identification.name);
    }
}




#[derive(Debug)]

struct Person {
    name:String,
    age: u8,
}

fn ask_pills() -> String {
    let mut pills = String::new();
    std::io::stdin().read_line(&mut pills).expect("Error while read the line");
    pills
}

impl Person {
    fn take_pills_sleep (&self) {
        if self.age >= 20 {
            let pills = &ask_pills();
            let value:i32  = pills.trim().parse().expect("Failed to parse integer");
            
            if value >= 6 {
               println!("Your dead!"); 
               return 
            }
            println!("Sleep for 12hr , Yep :) pills: {} ", value );
        
        } else{
            println!("Just quit coffee boy!, your young!");
        }
    }
}



#[derive(Debug)]
struct Point {
    x:f32,
    y:f32,
}


#[derive(Debug)]

struct Rectangle {
    left_top: Point,
    left_bottom: Point,
    base: f32,
    heigh: f32,
} 

impl  Rectangle {
    fn area(&self) -> f32 {
        let area:f32 = self.base * self.heigh;
        area
    }
    
    fn square(p:Point, v:f32) -> Rectangle {
        Rectangle {
            left_top: Point { x: p.x, y: p.y },
            left_bottom: Point {
                x: p.x,
                y: p.y - v,
            },
                base: v,
                heigh: v,
        }
    }
}



fn main() { 

let israel = PersonA(23, 34, 3.2);
    println!("{}", israel.0);
    println!("{}", israel.2);
    println!("{}", israel.1);
    println!("{:?}", israel);
    println!("{:#?}", israel);
    println!("\n\n");
    let number_id = String::from("010324034987");
    let name = String::from("Carlos Ernandez");
    
    let carlos = Student {
        identification:Identification {name, number_id},
        age: 23
    };

    println!("{:#?}", carlos);
    println!("{:#?}", carlos.identification);
    println!("{:?}", carlos.identification.name);
    println!("{:?}", carlos.identification.number_id);
    println!("{:?}", carlos.age);
    println!("\n\n");
    println!("{:?}", carlos.introduce());
    let adult:bool = carlos.is_adult();
    println!("Is an adult: {}",adult);
    println!("{:?}", carlos.identification.display_id());
    println!("\n\n");
    
    let name:String = String::from("Are you sure?");
    let age:u8 =17 ;
    let user = Person {name, age};
    println!("{user:?}");
    //user.take_pills_sleep();
    
    let x_1 = 3.3;
    let y_1 = 2.0;
    let x_2 = 10.3;
    let y_2 = 9.1;
    let x_3 = 0.3;
    let y_3 = 0.99;
    // Rentangle 
    
    let rect = Rectangle {
        left_top: Point {x: x_2, y: y_2},
        left_bottom: Point {x: x_1, y: y_1},
        base: 20.8,
        heigh: 12.3,
    }; 
 

    // objeto.metodo()
    // Tipo::funcion_asociada()

    println!("{:?}", rect);
    let p = Point {x: x_3, y: y_3};

    let rect_v2 = Rectangle::square(p, 34.2);
    println!("{:?}", rect_v2);
    let area = rect_v2.area();
    println!("{:?}", area);


}
