fn main() {
    println!("if, if-else");

    first();
    second();
    third();

}


fn first() {
    let no = 10;

    if no < 5 {
        println!("The condition is true");
    }

    else {
        let abc = 4+ 7;
        println!("{}", abc);
        println!("The condition is false");
    }
}


fn second() {
    let no = 2;

    if no % 4 == 0 {
        println!("Number is divisible by 4");
    }

    else if no % 6 == 0 {
        println!("Number is divisible by 6");
    }
    else if no % 8 == 0 {
        println!("Number is divisible by 8");
    }
    else {
        println!("This no is divisible by anyone");
    }
}

// short hand

fn third() {
    let condition = true;

    let number = if condition {5} else {0};

    println!("The number is {}", number);
}