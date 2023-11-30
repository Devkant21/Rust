fn main() {
    println!("Hello, world!");

    // always call the new functions inside the main function; otherwise it won't execute

    first_fn();

    single_params(23)
}

// simple function

fn first_fn() {
    println!("first_fn");
}

// passing a single parameter

fn single_params(x: i32) {
    println!("The value of x - {}",x);
}