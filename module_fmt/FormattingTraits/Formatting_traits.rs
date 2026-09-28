#![allow(unused)]
fn showtypestraits() {
    println!("nothing ftm:Diplay");
    println!("{:?}" ,"Debug");
    println!("{} {1:x?}", "Debug with lower case hexadecimal numbers" ,123);
    println!("{} {1:X?}", "Debug with Upper case hexadecimal numbers" ,8558);
    println!("o {:o}", 84); // -> 124
    println!("x {:x}", 15); // -> f
    println!("X, {:X}",12); // -> C
    println!("p, {:p}","kioPointer"); // -> {addr:..., metadata:num-characteres}
    println!("b, {:b}",7); // -> 111
    println!("e, {:e}",3874); // -> 3.874'e'3
    println!("E, {:E}",89867);  // -> 8.9867'E'4
    // return () its like void in C
}


// We can ommit the ''a' life-time for the compiler
fn identify(quoute:&str) -> String {
    quoute.to_uppercase()
}


fn add_more_text_reference(slice_text: &mut String) {
//fn add_more_text_reference<'a>(mut slice_text:&'a str) {
     
    slice_text.push_str("Negro, buena suerte!");

    // That don't make sense..., becaue I add to the variable
    // like this: [slice_text, point] -> [text_slice: metadata(...), addr(....)]
    //slice_text.push_str("Negro, buena suerte!");
    //let other_reference:&str = &slice_text;
   // println!("{}", slice_text);
   // println!("{}", other_reference);
}

// Important distiction:

//fn make_new(text: &str) -> String {
    // Reads borrowed text and creates a separate owned String
//}


//fn modify(text: &mut String) {
    // Can append and modify the owned String
//}


//fn read(text: &str) {
    // Can read; cannot modify the text
//}


// Struct Addiction
struct Addiction<'a> {
    app_name:String, // String that struct owns 
    // slice-string borrow the string: &str -> unsized type (The reference contains the text’s address and length)
    // 'a -> lifetime (compile-time name for the period during which that borrowed reference is valid.), 
    app_oppinion:&'a str ,
    level_of_dopamine:i32, // signed 32 bytes
    percentage_of_decline:i16, // signed 16 bytes 
    brain_damage:bool, // True/False
}

// Manual Debug

impl std::fmt::Debug for Addiction<'_> {
 fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
     
     f.debug_struct("Addiction")
    .field("app_name", &self.app_name)
    .field("app_oppinion", &self.app_oppinion)
    .field("level_of_dopamine", &self.level_of_dopamine)
    .field("percentage_of_decline",&self.percentage_of_decline)
    .field("brain_damage",&self.brain_damage)
    .finish()
  
 }
}



fn main () {
    // Show the types of trains
    showtypestraits();
    println!("You know, that we can make or change the behavior of the trinas");
    // &str 
    // let saludo_original:String = "Hola soy yo el orignal".to_string();
    let saludo_original_v2 = String::from("Hola soy yo el orignal");

    let saludo="Hola fernando!"; // -> saludo:&str -> String
    let hola:&str = saludo;
    
    let copy_saludo_original:&str = &saludo_original_v2;

    println!("{}", hola);
    println!("{}", saludo);
    println!("{}", saludo_original_v2);
    println!("{}", copy_saludo_original);

    // Function identify
    let quote = String::from("Ey!, Don't be distracted!, you gonna died soon");
    // We send the reference of the string.
    let upper_quote=identify(&quote);
    println!("{}", upper_quote);

    // add_more_text
    let mut text_slice = String::from("Gato");
    // let reference = &text_slice;
    add_more_text_reference(&mut text_slice);  // The 'text_slice' end when the function end... even if you send
                                // a reference.. it would give you a error , because the original
                                // addr doen't exist 
    //let reference:&str = &text_slice; 
   println!("{}", text_slice);

    let mut text = String::from("Hola");

    {
        let reference = &text;
        println!("{}", reference);
    } // reference's borrow ends here

    text.push_str(" mundo");

    println!("{}", text);
    //println!("{}", reference);
    
    // Struct: Addition
    let reviews = String::from("The best app\n I need it for my work\n It help me to focus dah");
    let app = String::from("Youtbue");
    
    let my_addiction = Addiction {
        app_name:app, // String that struct owns 
        app_oppinion:&reviews,
        level_of_dopamine:99, // signed 32 bytes
        percentage_of_decline:80, // signed 16 bytes 
        brain_damage:true, // True/False
    };
    // We need to implement debug
    println!("{my_addiction:?}");
    println!("{my_addiction:#?}");
}
