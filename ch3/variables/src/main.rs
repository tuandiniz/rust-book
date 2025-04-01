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
    
    let _val: u32 = "42".parse().expect("Not a number!");

    let mut x = 2.0f64;

    let y = 3.0f32;

    println!("x + y = {}", y + x as f32);

    println!("Divide by zero: {}", 2.0 / 0.0);
    println!("Divide zero by zero: {}", 0.0 / 0.0);
    println!("Integer overflow: {}", u32::MAX - 2);
    
    // _val += 1;
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    
    println!("Nice chars: {c} {z} {heart_eyed_cat}");
    let val_char = b'C';
    println!("Value: {}", val_char);
    
    // Tuples
    let typed_tuple: (i32, f32, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    
    let (x, y, z) = tup;
    
    println!("The value of y is {y} and {typed_tuple:?}");
    println!("{}, {}, {}", tup.0, tup.1, tup.2)
}
