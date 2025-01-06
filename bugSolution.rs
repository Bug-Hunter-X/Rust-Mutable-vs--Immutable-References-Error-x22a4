fn main() {
    let mut x = 5;
    let y = &mut x;
    *y = 10;
    let z = &x;
    println!("x = {}", x); //Prints 10
    println!("z = {}", *z); //Prints 10
    //This is fine as it's a mutable reference
    let mut a = 10;
    let b = &mut a;
    *b = 20; 
    println!("a = {}", a); //Prints 20 
    //This is also fine as it's a mutable reference 
    let mut c = 20;
    let d = &mut c; 
    *d = 30; //This is now fixed since d is a mutable reference 
    println!("c = {}", c); //Prints 30
    
} 