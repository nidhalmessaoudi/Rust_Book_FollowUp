use std::f32::consts::E;

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option<T> enum
    let some_number = Some(5);
    let some_char = Some('N');

    let absent_number: Option<i32> = None;
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
    }
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String
// }
