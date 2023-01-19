use crate::car::{Car, Age, Convertible, Transmission, Condition};
use rand::Rng;
use std::collections::HashMap;

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
pub fn car_factory(color: String, transmission: Transmission, convertible: Convertible, condition: Age) -> Car {

    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type 
    // - Print details for New or Used car based on roof type
    if condition.0 == Condition::Used {
        print!("Prepare a used car: {:?}, {:?}, {}, {} miles\n", transmission, convertible, color, condition.1);
    } else {
        print!("Build a new car: {:?}, {:?}, {}, {} miles\n", transmission, convertible, color, condition.1);
    }
             
    // Use the values of the input arguments
    // All new cars always have zero mileage
    let car = Car {
        color: String::from(&color),
        transmission: Transmission::from(transmission),
        convertible: Convertible::from(convertible),
        mileage: condition,
    };

    car

}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
pub fn car_quality (miles: u32) -> (Condition, u32) {

    let mut condition = Condition::New;
    // Check milage and derive Age from it
    if miles > 0 {
        condition = Condition::Used;
    }
    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    let quality: (Condition, u32) = (condition, miles);
    // Return the completed tuple to the caller
    quality
}

// Function to generate an array of random integers for car attributes
// - max_color (u32)
// - max_milage (u32)
// return a tuple with random configuration
pub fn random_configuration(max_color: u32, max_milage: u32) -> (u32, Transmission, Convertible, u32) {
    let mut rng = rand::thread_rng();

    // generates random properties for: color, transmission, mileage
    let random_numbers = [
        rng.gen_range(0..max_color), 
        rng.gen_range(0..3), 
        rng.gen_range(0..max_milage)];
    
    // generates random property for convertible
    let random_boolean = rng.gen_bool(0.5);

    // convert integer to enum value for transmission
    let transmission_type = match random_numbers[1] {
        0 => Transmission::Manual,
        1 => Transmission::SemiAuto,
        2 => Transmission::Automatic,
        _ => Transmission::Manual,
    };

    // convert integer to enum value for convertible
    let roof_type = match random_boolean {
        true => Convertible::HardTop,
        false => Convertible::SoftTop,
    };

    (random_numbers[0], transmission_type, roof_type, random_numbers[2])
}

// Takes a reference to a HashMap of cars and prints the entire list
pub fn print_lot(orders: &HashMap<u32, Car>){
    for (key, value) in orders.iter() {
        println!("{}: {:?}", key, value);
    }
}

// Takes a reference to a HashMap of cars and a car order number
// Invokes the print_lot function to print the entire list
pub fn print_order(orders: &HashMap<u32, Car>){

    println!("Cars prepared: {}", orders.len());
    print!("-------------------------\n");
    print_lot(&orders);
}

// Begin of tests for the factory module
// -------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_factory() {
        assert_eq!(1,1);
    }
}