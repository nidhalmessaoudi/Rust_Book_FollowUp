fn main() {
    println!("Hello, world!");

    another_func(789, 'N');

    // Every block in Rust is an expression
    // That is allowed to have statements within.
    let y = {
        let x = 46;
        x - 40
    };

    println!("y: {y}");

    let mut x = seven();
    println!("x: {x}");

    x = plus_one({
        let y = 40;
        y - x
    });
    println!("x: {x}");
}

fn another_func(num: i32, label: char) {
    println!("Hello from another func! ID is: {num}{label}");
}

fn seven() -> i32 {
    7
}

fn plus_one(num: i32) -> i32 {
    num + 1
}
