use std::io;

// This is my first rust application. Hello World!
// Takes an ,command line input in Kg and converts it into lb
// Simple as that.
fn main() {
    println!("Enter your Weight in Kg:");
    let mut input = String::new();


    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    let kg_weight = convert_kg_lb(weight);

    println!("Weight {}kg", kg_weight);
}

fn convert_kg_lb(weight: f32) -> f32{
    return weight * 2.205; // rounded 3 decimals
}

