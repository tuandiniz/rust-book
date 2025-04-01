fn main() {
    println!("Hello, world!");
    another_function(32);
    print_labeled_measurement(32, 's');
    
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}