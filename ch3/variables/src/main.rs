const SECONDS_IN_HOUR:u32 = 60 * 60;

fn main() {
    let mut x = 3;
    println!("Value of x: {x}");

    {
        let x = x + 3;
        println!("Value of x: {x}");
    }
    
    x = 4;
    println!("Value of x: {x}");

    println!("Seconds in an hour: {SECONDS_IN_HOUR}");
    
    let val: u32 = "42".parse().expect("Not a number!");
}
