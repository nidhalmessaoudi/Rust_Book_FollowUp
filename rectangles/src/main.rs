fn main() {
    let scale = 2;
    let rect1 = Rectangle {width: 30, height: dbg!(25 * scale)};

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(30);

    println!("Square SQ has a size of {}", sq.width);

    if rect1.width() {
        println!("The rectangle has a positive width of: {}", rect1.width);
    }

    dbg!(&rect1);
    println!("The area of the rectangle is {} square pixels!", rect1.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Struct methods
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

// Non-readable way
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// With tuples
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Passing a struct as a param
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.height * rectangle.width
// }
