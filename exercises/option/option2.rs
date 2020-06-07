// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// I AM NOT DONE

fn main() {
    let optional_value = Some(String::from("rustlings"));
    if let value = optional_value {
        println!("the value of optional value is: {}", value.unwrap()); // We need to unwrap the Some to get it's value
    } else {
        println!("The optional value doesn't contain anything!");
    }

    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let value = optional_values_vec.pop() {
        // if value == None {
            // println!("Done");
        // } else {
            println!("current value: {}", value.unwrap().unwrap());
        // }
    }
}
