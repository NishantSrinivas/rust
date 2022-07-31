// use std::mem;

fn main() {
    println!("Hello Rust!");

    // // declaring immuatable var 
    // let intu8: u8 = 255;
    // println!("intu8 = {}, with size {} bytes",intu8,mem::size_of_val(&intu8));

    // let intu32: u32 = 65535;
    // println!("intu32 = {}, with size {} bytes",intu32,mem::size_of_val(&intu32));

    // // let intu64 = 12345678901234567890; type inferencing didn't choose u64, instead chose i32
    // let intu64: u64 = 12345678901234567890;
    // println!("intu64 = {}, with size {} bytes",intu64,mem::size_of_val(&intu64));

    // let testvar: usize = 100;
    // println!("testvar = {}, with size {} bytes, {}-bits",testvar,mem::size_of_val(&testvar),mem::size_of_val(&testvar)*8);

    // // declaring some mutable variables
    // let mut a = 10;
    // println!("a before {}",a);

    // a = a * 10;
    // println!("a after {}",a);

    // let mut boolean = true;
    // println!("boolean = {}",boolean);
    // boolean = false;
    // println!("boolean = {}",boolean);
    // println!("{}",&boolean);

    // playing with loops

    // for x in 1..=10
    // {
    //     println!("x => {}",x);
    // }

    // for (i,y) in (1..=20).enumerate()
    // {
    //     println!("{}: {}",i,y);
    // }

    // for (_,y) in (1..=20).enumerate()
    // {
    //     println!("{}",y);
    // }
    
    // let mut x = 0;
    // while x <= 10
    // {
    //     println!("{}",x);
    //     x+=1;
    // }

    // The never ending loop xD
    // let mut y = 0;
    // loop
    // {
    //     if y == 5
    //     {
    //         continue;
    //     }
    //     println!("{}",y);
    //     y += 1;
    //     if y==10 
    //     {
    //         break;
    //     }
    // }

    // let x: u8 = 10;
    // let num: &str = match x 
    // {
    //     1 => "one",
    //     2 => "two",
    //     3 => "three",
    //     4 => "four",
    //     5 => "five",
    //     6 => "six",
    //     7 => "seven",
    //     8 => "eight",
    //     9 => "nine",
    //     10 => "ten",
    //     _ => "out of bounds"
    // };

    // println!("{}",num);

}
