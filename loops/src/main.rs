fn main() {
    // loop {
    //     println!("Successful again and  again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;

    'counting_up: loop {
        println!("Count is {count}");

        let mut remaining = 10;

        loop {
            println!("Remaining is {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count is {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!");

    let first_arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Slower and error prone
    while index < first_arr.len() {
        println!("Value is {}", first_arr[index]);

        index += 1;
    }

    // Faster and safer
    for element in first_arr {
        println!("Element value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!");
}
