#[allow(dead_code)]
// use std::mem;

// struct Point {
//     x: f64,
//     y: f64,
// }

// struct Line {
//     start: Point,
//     end: Point,
// }

// fn create_line(_start: Point, _end: Point) -> Line {
//     let line: Line = Line {
//         start: _start,
//         end: _end,
//     };
//     return line;
// }

// enum Shapes
// {
//     Circle,
//     Square,
//     Rectangle,
//     Triangle
// }

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

    // playing with data structures

    // let p1: Point = Point { x: 10.0, y: 20.0 };
    // let p2: Point = Point { x: 20.0, y: 30.0 };

    // let newline = create_line(p1, p2);

    // println!(
    //     "Line starts at ({},{}) and ends at ({},{})",
    //     newline.start.x, newline.start.y, newline.end.x, newline.end.y
    // );

    // let my_shape: Shapes = Shapes::Triangle;

    // match my_shape
    // {
    //     Shapes::Triangle => {println!("It's a Triangle")},
    //     Shapes::Rectangle => {println!("It's a Rectangle")},
    //     Shapes::Square => {println!("It's a Square")},
    //     Shapes::Circle => {println!("It's a Circle")}
    // }

    // Option type
    // let x = 10;
    // let y = 5;
    // let y = 0;

    // let z = if y == 0 { None } else { Some(x / y) };

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

    // playing with data structures

    // let p1: Point = Point { x: 10.0, y: 20.0 };
    // let p2: Point = Point { x: 20.0, y: 30.0 };

    // let newline = create_line(p1, p2);

    // println!(
    //     "Line starts at ({},{}) and ends at ({},{})",
    //     newline.start.x, newline.start.y, newline.end.x, newline.end.y
    // );

    // let my_shape: Shapes = Shapes::Triangle;

    // match my_shape
    // {
    //     Shapes::Triangle => {println!("It's a Triangle")},
    //     Shapes::Rectangle => {println!("It's a Rectangle")},
    //     Shapes::Square => {println!("It's a Square")},
    //     Shapes::Circle => {println!("It's a Circle")}
    // }

    // Option type
    // let x = 10;
    // let y = 5;
    // let y = 0;

    // let z = if y == 0 { None } else { Some(x / y) };

    // match z {
    //     Some(v) => println!("{}/{} = {}", x, y, v),
    //     None => println!("divide be zero not possible"),
    // };

    // Arrays
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    let arr2: [u8; 10] = [1; 10];
    println!("arr2 : {:?}", arr2);
    println!("arr2 size: {}", arr2.len());

    let arr3 = &arr2[3..7];
    println!("arr3 : {:?}", arr3);
}
