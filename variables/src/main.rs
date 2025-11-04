fn main() {
    let  x = 5;
    println!("The value of x is: {}", x);
    {
        let x = 76;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);
}
