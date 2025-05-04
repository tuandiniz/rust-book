fn main() {
    let mut s = String::from("Hello, world!");
    
    let subs = first_word(&s);

    println!("First word: {subs}");
    s.clear();
    println!("Another: {}", first_word("Now this is a good test"));
    println!("Another: {}", first_word(&"Now this is a test"[4..]));
    
    let a = [1, 2, 3, 4, 5];
    
    println!("int slice: {:?}", &a[1..=3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}