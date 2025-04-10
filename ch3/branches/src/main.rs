fn main() {
    let number = 14;

    let message =  if number < 5 {
        "condition was true"
    } else {
        "condition was false"
    };
    println!("{}", message);

    if number != 0 {
        println!("{} was something other than zero!", number);
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {5} else {"six"};

    println!("The value of number is {number}");
}
