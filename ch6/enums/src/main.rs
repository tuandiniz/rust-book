fn main() {
    let localhost = IpAddrType::V4(String::from("127.0.0.1"));
    let local_loop = IpAddrType::V6(String::from("::0"));
    
    println!("v4: {localhost:?}, v6: {local_loop:?}");
    
    let addr = match localhost {
        IpAddrType::V4(v) => v,
        _ => String::from("Whatever") 
    };
    
    println!("Address: {addr}");

    let other_addr = match local_loop {
        IpAddrType::V6(v) => v,
        _ => String::from("Whatever")
    };

    println!("Other Address: {other_addr}");
    
    let msg = Message::Write(String::from("This is a message from the deep"));
    msg.call();
    
    let some_number = Some(9);
    let some_char = Some('e');
    
    let absent_number: Option<i32> = None;
}

#[derive(Debug)]
enum IpAddrType {
    V4(String),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call (&self) {
        println!("A call");
    }
}