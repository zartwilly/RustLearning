use std::io;

fn main() {
    println!("Hello, world!");
    
    // scalar 
    let x: u32 = 2;
    let y: u32 = 4;
    
    let i: f32 = 10.0;
    let j: f32 = 20.1;
    
    let res: f32 = j / i;
    //res = 3.14;
    
    let trunc_int = -5 / 2;
    let trunc_float = -5. / 2.;
    
    let f: bool = false;
    
    let c: char = 'Z';
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let i = tup.0;
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    
    
    println!("x = {x}, y = {y}, res = {res}, trunc_int = {trunc_int}, trunc_fl = {trunc_float}");
    println!("f = {f}, c={c}, i={i}, a={first}");
    
    
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    
    
    
}
