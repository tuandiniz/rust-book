fn main() {
    let x = 5;
    let y = x;
    println!("Numbers now {x} and {y}");

    let s = "hello";
    let t = s;

    println!("Hello {s} and {t}");

    let mut s = String::from("Hello");
    s = String::from("Hola");

    println!("{s} World!");

    let mut t = s.clone();
    t.push_str(" querido");
    println!("Again {s} and {t}");

    takes_ownership(s);
    //println!("{s} is not alive");

    makes_copy(x);
    println!("{x} is alive");

    let mut new_string = gives_ownership();
    println!("New: {new_string}");

    new_string = takes_and_gives_back(new_string);
    println!("Continues new: {new_string}");

    let (new_string, len) = calculate_length(new_string);
    println!("\"{new_string}\" has length {len}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("Take It");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}