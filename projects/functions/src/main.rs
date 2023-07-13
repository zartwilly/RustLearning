use std::io;

fn main() {
    println!("Hello, world!");
    
    let x = five();
    let y = plus_one(5);
    println!("The value of x is {x} and y is {y}");
    
    let mut x_val = String::new();
    io::stdin()
        .read_line(&mut x_val)
        .expect("Failed to read line");
        
    //let x_val: usize = x_val
    let x_val: i32 = x_val
        .trim()
        .parse()
        .expect("x_val={x_val} entered was not a number");
    
    another_function(x_val);
    print_labeled_measurement(x_val, 'h');
}

fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32{
    x+1
}
