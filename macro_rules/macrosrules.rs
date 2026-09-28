
macro_rules! say_hello {
    ($name:expr) => {
        println!(" macro_rules {} \n", $name);
    };
}

macro_rules! sum {
    ($($val:expr),+) => {
        {
            let mut tmp_sum=0;
            println!("\n");
          
            // Loop and $val: represent a list of values...
            $(
               println!(" tmp:{} + val:{}", tmp_sum, $val);
               tmp_sum+=$val;
            )*
           
            tmp_sum
        }
    };
}

fn main () {
    let sum_num:u32 = sum!(1,2,3,4,5);
    println!("\n Suma de (1,2,3,4,5) {}", sum_num);
    say_hello!(" Carmen");
}
