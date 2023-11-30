fn main() {
    println!("Variables");

    // to declare a Variables

    // Always terminate the line using semi-colon

    // Variables are immutable in Rust

    let a = 77;

    println!("{a}");


    // to print using the stringify method

    //  this is the best practise method
    println!("{}",a);


    // to declare a mutable Variables

    let mut b = 56;

    println!("{}", b); 

    b = 100;

    println!("{}", b);
}
