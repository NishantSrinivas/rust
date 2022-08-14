#[allow(dead_code)]
#[allow(unused_imports)]
// use std::collections::HashMap;
// use std::collections::HashSet;
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

// fn match_int_pattern(i: u8) {
//     match i {
//         1 => println!("it's one"),
//         2..=5 => println!("it's some number btw 2 and 5"),
//         _ if (i % 6 == 0) => println!("it's some number divisible by 6"),
//         _ if (i % 10 == 0) => println!("it's some number divisible by 10"),
//         _ => println!("some value"),
//     }
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
    // let arr = [1, 2, 3, 4, 5];
    // println!("{:?}", arr);

    // let arr2: [u8; 10] = [1; 10];
    // println!("arr2 : {:?}", arr2);
    // println!("arr2 size: {}", arr2.len());

    // let arr3 = &arr2[3..7];
    // println!("arr3 : {:?}", arr3);

    // Tuples
    // let mut tuple1 = (10, false);
    // println!("{} and {}",tuple1.0, tuple1.1);

    // let (a,b) = tuple1;
    // println!("a is {}",a);
    // println!("b is {}",b);

    // tuple1.0 = 12;
    // tuple1.1 = true;
    // println!("{}",tuple1.0);

    // Pattern matching
    // match tuple1
    // {
    //     (10,false) => println!("10 and false"),
    //     (12,false) => println!("12 and false"),
    //     (_,true) => println!("something with true"),
    //     _ => println!("something")
    // }

    // for i in 1..21 {
    //     match_int_pattern(i);
    // }

    // Vec

    // let mut vector = Vec::new();

    // for i in 1..6 {
    //     vector.push(i);
    // }

    // println!("vector = {:?}", vector);
    // println!("vector size: {}", vector.len());

    // let x = vector.pop();
    // match x {
    //     Some(x) => println!("we have some value {}", x),
    //     None => println!("vector didn't contain any value"),
    // }

    // println!("vector = {:?}", vector);
    // println!("vector size: {}", vector.len());

    // for _ in 0..4 {
    //     vector.pop();
    // }

    // println!("vector = {:?}", vector);
    // println!("vector size: {}", vector.len());

    // let y = vector.pop();
    // match y {
    //     Some(y) => println!("we have some value {}", y),
    //     None => println!("vector didn't contain any value"),
    // }

    // HashMap
    // let mut my_map = HashMap::new();

    // my_map.insert("Cat", 4);
    // my_map.insert("Duck", 2);

    // println!("my_map = {:?}", my_map);
    // println!("my_map = {}", my_map.len());

    // for (animal, leg_count) in &my_map {
    //     println!("{} has {} legs", animal, leg_count);
    // }

    // println!("Cat has {} legs", my_map["Cat"]);
    // println!("Human has {} legs",my_map["Human"]); // will cause error

    // my_map.insert("Dog",5);

    // println!("my_map = {:?}",my_map);
    // println!("my_map = {}",my_map.len());

    // my_map["Dog"] = 4; // will cause error

    // HashSet
    // let mut my_set = HashSet::new();

    // my_set.insert("A");
    // println!("my_set = {:?}",my_set);

    // my_set.insert("B");
    // my_set.insert("C");
    // my_set.insert("D");
    // my_set.insert("E");
    // my_set.insert("F");

    // println!("my_set = {:?}",my_set);

    // if my_set.contains("A")
    // {
    //     println!("my_set has A in it");
    // }

    // if !my_set.contains("X")
    // {
    //     println!("my_set doesn't have X in it");
    // }

    // let mut my_set2:HashSet<_> = (1..=10).collect();
    // if my_set2.insert(11)
    // {
    //     println!("insert ok");
    // }
    // println!("{:?}",my_set2);

    // Iterator
    
    // let vec = vec![1,2,3,4,5];

    // for i in vec.iter()
    // {
    //     println!("{}",i);
    // }

    // for j in vec.iter().rev()
    // {
    //     println!("{}",j);
    // }

    // for (ind,k) in vec.iter().rev().enumerate()
    // {
    //     println!("indx : {}, val : {}",ind,k);
    // }

    // Strings

    // let string:String = "Hello World".to_string(); // converting from &str -> String
    
    // for c in string.chars()
    // {
    //     println!("{}",c);
    // }

    // for c in string.chars().rev()
    // {
    //     println!("{}",c);
    // }

    // for (indx,c) in string.chars().enumerate()
    // {
    //     println!("{} -> {}",indx,c);
    // }

    // string = string + ", Welcome to Rust programming!";

    // let greetings:String = format!("Rust wants to say {}",string);
    // let name = "Nishant".to_string();    
    // println!("{}, from {_name}'s program.",greetings, _name = name);
    
    // println!("{}",string);
}
