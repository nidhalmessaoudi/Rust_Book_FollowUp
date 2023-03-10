fn main() {
    let mut s1 = String::from("Software");

    {
        let r1 = &mut s1;

        println!("{r1}");
    }
    
    // Mutable reference to s1
    change(&mut s1);

    // Referencing to s1 with &s1: Borrowing the value of s1 but not owning it.
    let len  = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let s2 = no_dangle();

    println!("{s2}");
}

fn change(s: &mut String) {
    s.push_str(" is beautiful!");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Dangling pointer which will cause undefined behavior
// fn dangle() -> &String {
//     let s = String::from("Hi!");

//     &s
// }

// Safe code by moving ownership
fn no_dangle() -> String {
    let s = String::from("Hi!");

    s
}
