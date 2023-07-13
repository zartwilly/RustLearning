fn main() {
    println!("Hello, world!");
    
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("new x = {x}");
    
    {
        let x = x * 2;
        println!("shadowed  x = {x}");
    }
    println!("x = {x}");
}
