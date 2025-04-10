fn main() {
    let mut counter = 0;

    'counting_up: loop {
        println!("count = {counter}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        counter += 1;
    };

    println!("End count: {counter}");

    let mut number = 3;

    while number > 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let arr = [10, 20, 30, 40, 50];

    for element in 1..10 {
        println!("The value is: {element}");
    }

    for element in (1..=4).rev() {
        println!("Descending: {element}");
    }
}
