fn main() {
    let scale = 2;
    let rect1 = Rectangle {width: 30, height: dbg!(50 * scale)};

    dbg!(&rect1);
    println!("The area of the rectangle is {} square pixels!", area(&rect1));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Non-readable way
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// With tuples
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// With structs
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
