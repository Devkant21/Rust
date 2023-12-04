

fn main() {
    println!("Data types in Rust");

    // Scalar types -> types where we can store single value
    // Integer, Strings, Boolean, Floating, char

    // Integer -> length (8bit, 16, 32, 64, 128, arch)



    let no = 2;
    println!("{}", no);

    // Boolean

    let is_male = true;
    println!("{}", is_male);

    let char = "abc";
    println!("{}", char);

    let dec =78.99;
    println!("{}", dec);



    // Compound types -> types where we can store multiple data types
    // Arrays, tuples, dictionary,

    //  tuples

    let mut tup = (32,45,66.77);

    println!("{:?}", tup);

    println!("{}", tup.1);
    println!("{}", tup.2);

    tup.2 = 88.7;

    println!("{:?}", tup);

    // Arrays

    let array = [34, 66, 09, 87];
    println!("{:?}", array);

    println!("{}", array[3]);
}
