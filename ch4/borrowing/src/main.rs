fn main() {
    let mut s1 = String::from("hello");
    
    let len = calculate_length(&s1);
    
    println!("The length of {s1} is {len}");
    
    change(&mut s1);
    println!("{s1}");
    
    let r1 = &s1;
    let r2 = &s1;
    
    println!("Both strings: {r1}, {r2}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn change(some_string: &mut String) -> () {
    some_string.push_str(" And I say more");
}

fn get_string_ref() -> &String {
    let str = String::from("String to go!");
    
    &str
}