// This is my solution for the coding exercise from the Microsoft Learn Rust path

mod car;
mod factory;

use car::Car;
use factory::car_factory;
use factory::car_quality;
use factory::random_configuration;

use rand::Rng;
use std::collections::HashMap;

use crate::factory::print_order;


fn main() {
    // create car array with available colors
    let mut colors = ["blue", "green", "red", "silver"];

    // cast length of colors array to an integer
    let colors_len = colors.len() as u32;

    // Initialize a hash map to store the cars
    // - Key: Car order_number, i32
    // - Value: Car properties, Car struct
    let mut orders: HashMap<u32, Car> = HashMap::new();

    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    // Create a new random configuration for each car
    // example: [2, Transimission, Roof, 3000] = [color, transmission, convertible, mileage]
    // input: [color, mileage] (transmission, convertible) are fixed enums

    // Create new cars with random specs based on user input

    //get user input from command line
    let mut input = String::new();
    println!("How many cars do you want to build?");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut input: u32 = input
        .trim()
        .parse()
        .expect("Please provide the amount of cars (Integer)!");

    let mut order_count: u32 = 0;

    // Begin of car factory initialization
    //
    // Create a new random configuration for each car
    let mut configuration = random_configuration(colors_len, 10000);

    let mut color = String::from(colors[configuration.0 as usize]);
    let mut engine = configuration.1;
    let mut convertible = configuration.2;
    let mut age = car_quality(configuration.3);

    // End of car factory initialization

    // Begin of car factory loop

    while order_count < input {
        let car = car_factory(color, engine, convertible, age);
        println!("Car is ready!");
        configuration = random_configuration(colors_len, 10000);
        color = String::from(colors[configuration.0 as usize]);
        engine = configuration.1;
        convertible = configuration.2;
        age = car_quality(configuration.3);
        order_count += 1;
        orders.insert(order_count, car);
    }

    // End of car factory loop

    // Begin of car factory output
    print_order(&orders);
    // End of car factory output

}


// Begin of tests for the main module
// -------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_main() {
        assert_eq!(1,1);
    }
}