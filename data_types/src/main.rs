fn main() {
    ////// Scalar Types
    // Integer types
    let x: i64 = -1257;
    let y: u32 = 15;
    let z: usize = 200000;
    println!("{x}, {y}, {z}");

    // Float types
    let x: f32 = 12.78549;
    let y: f64 = 154_984.234_56;
    println!("{x}, {y}");

    // Char type
    let my_first_char: char = 'N';
    let my_emoji = '💪';
    let my_first_arab_char = 'ن';
    println!("{my_first_char}, {my_emoji}, {my_first_arab_char}");

    // Bool type
    let is_successful = true;
    println!("{is_successful}");

    ////// Compound Types
    // Tuple type
    let my_tup = (7, "Nidhal", "Software", '🔥');
    let (w, x, y, z) = my_tup;
    println!("{w}, {x}, {y}, {z}");

    // Array type
    let my_arr = [20; 1000];
    println!("{}", my_arr[999]);
}