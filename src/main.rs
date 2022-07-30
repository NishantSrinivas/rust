use std::mem;

fn main() {
    println!("Hello Rust!");

    // declaring immuatable var 
    let intu8: u8 = 255;
    println!("intu8 = {}, with size {} bytes",intu8,mem::size_of_val(&intu8));

    let intu32: u32 = 65535;
    println!("intu32 = {}, with size {} bytes",intu32,mem::size_of_val(&intu32));

    // let intu64 = 12345678901234567890; type inferencing didn't choose u64, instead chose i32
    let intu64: u64 = 12345678901234567890;
    println!("intu64 = {}, with size {} bytes",intu64,mem::size_of_val(&intu64));

    let testvar: usize = 100;
    println!("testvar = {}, with size {} bytes, {}-bits",testvar,mem::size_of_val(&testvar),mem::size_of_val(&testvar)*8);

    // declaring some mutable variables
    let mut a = 10;
    println!("a before {}",a);

    a = a * 10;
    println!("a after {}",a);

    let mut boolean = true;
    println!("boolean = {}",boolean);
    boolean = false;
    println!("boolean = {}",boolean);
    println!("{}",&boolean);
}
