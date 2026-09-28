fn main() {

    // With '{}' we can replace with any value, called stringified
    println!("{}", "HOla como vas? ");
    println!("{}", "HOla como vas? ");
   
    let _hola = format!("Hi\n");
    print!("{}", _hola);
    
    let _sum = 2+3;
    println!("{0} + {1} is: {2} ", 2 , 3, _sum);
    println!("{name}{lastname} ", name="Naranja", lastname="raja");
    println!("{:b}",8234);
    println!("{:o}",8234);
    

    let iteration1=format!("{1} {} {0} {}", 1, 2); // => "2 1 1 2"
    let iteration2=format!("{2} {} {1} {} {0} {}", 1, 2, 3); // => "3 1 2 2 1 3"
    println!("{}" , iteration2);
    let objet=format!("{a} {c} {b}", a="a", b='b', c=3);

    fn make_label(a:u32, b: &str) -> String{
        format!("{a} {b}")
    }

    let label=make_label(934, "Ticket");
    println!("{}", label);

    // Precision
    println!("Hello {0:.5}", 0.10);
    println!("Hello {:.*}", 3, 0.10);
    println!("Hello {1:.*}", 8, 0.10);
    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);
    
    // Limit decimals
    let pi = 3.141592;
    println!("{0:.3}", pi)
    
    println!("{}, `{name:.*}` has 3 fractional digits", "Hello", 3, name=1234.56);
    println!("{}, `{name:.*}` has 3 characters", "Hello", 5, name="1234.56"); // 1234. , Characters
    println!("{}, `{name:>8.*}` has 3 right-aligned characters", "Hello", 3, name="1234.56" );

    // Escaping with '{ or }'
    assert_eq!(format!("Hello {{}}"), "Hello {}");
    assert_eq!(format!("{{ Hello"), "{ Hello");
}
