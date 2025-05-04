#[derive(Debug)]
struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

struct Point(u64, u64, u64);
struct Color(u64, u64, u64);

fn main() {
    let user1 = build_user(String::from("Tuan"), String::from("tuan-whatever@gmail.com"),
        true, 1);
    
    println!("User: {:?}", user1);

    let user2 = User {
        name: String::from("Chico justino"),
        ..user1
    };

    println!("User: {:?}", user2);
    println!("User 1 name: {}", user1.name);
    
    let point = Point(32, 31, 1);
    let color = Color(234, 10, 120);
    
    let Point(x, y, z) = point;
    let Color(r, g, b) = color;
    
    println!("Color {:?}", (r, g, b));
    println!("Point {:?}", (x, y, z));
}

fn build_user(name: String, email: String, active: bool, sign_in_count: u64) -> User {
    User { name, email, active, sign_in_count }
}