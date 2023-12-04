fn main() {
    println!("Loops");

    first();
    second();
    third();
    fourth();
}

// Types of loops in Rust

// loops
// while
// for

//  Continuosly running loop

// fn first() {
//     loop {
//         println!("Devkant Swargiary")
//     }
// }

fn first() {
    let mut x = 0;

    loop {
        x += 1;
        println!("x = {}", x);

        if x == 5 {
            println!("Printed successfully");
            break;
        }
    }
}

fn second() {
    let mut number = 0;

    while number != 0 {
        println!("{}", number);

        number == 1;
    }

    println!("Hello");
}

fn third() {
    let a = [2, 4, 6, 8, 9, 5, 4, 6, 9 , 900, 98];

    let mut index = 0;

    while index < 10 {
        println!("the value is : {}", a[index]);

        index += 1;
    }
}

//  for loop

fn fourth() {
    let mut x = 0;
    for x in 1..11 {
        if x == 5 {
            continue;
        }
        println!("x is {}", x);
    }
}
