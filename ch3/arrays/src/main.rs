use std::io;

fn main() {
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    // println!("{a:?}");

    let _same = [3; 8];
    // println!("{same:?}");
    
    let _strings = ["This", "is", "a", "string"];
    
    println!("Please enter an array index.");
    
    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    
    let element = a[index];
    
    println!("The value of the element at index {index} is: {element}");
}
