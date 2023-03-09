fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("Rustacian");

    let (s3, len) = calculate_length(s2);

    println!("{s1}, {s3} has a length of {len}");
}

fn gives_ownership() -> String {
    let some_string = String::from("Software");
    some_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
