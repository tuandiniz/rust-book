fn main() -> () {
    println!("Hello, world!");
    another_function(32);
    print_labeled_measurement(32, 's');
    
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is: {y}");

    let x = five();
    println!("Five: {x}");
    println!("Plus one: {}", plus_one(x));
}

fn another_function(x: i32) -> () {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}