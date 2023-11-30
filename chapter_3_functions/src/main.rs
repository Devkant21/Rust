fn main() {
    println!("Hello, world!");

    // always call the new functions inside the main function; otherwise it won't execute

    first_fn();

    single_params(23);
    multi_params(33, 'f');
    expression();

    let value = return_fn();
    println!("The value of return function is : {}", value);
}

// simple function

fn first_fn() {
    println!("first_fn");
}

// passing a single parameter

fn single_params(x: i32) {
    println!("The value of x - {}",x);
}

fn multi_params(x: i32, y: char) {
    println!("The value of x - {x} & The value of y - {y}")
}

fn expression() {
    let x = {
        let y = 9;
        y + 1
    };

    println!("The value of x - {}",x);
}

// to return a function
// using arrow function we return the value of the function

fn return_fn() -> i32 {
    66 + 88
}