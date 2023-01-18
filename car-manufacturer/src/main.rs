// This is my solution for the coding exercise

use std::ops::Deref;

fn main(){

// Create car color array
let mut colors = ["blue", "green", "red", "silver"];
let mut engine = Transmission::Manual;

// We have orders for three new cars!
// We'll declare a mutable car variable and reuse it for all the cars

    let mut car = car_factory(String::from(colors[2]), engine, false, car_quality(0));

    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[3]), engine, true, car_quality(565));

    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[0]), engine, false, car_quality(3000));
}


struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: age,
}

type age = (Age, u32);

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool, condition: age) -> Car {


    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type 
    // - Print details for New or Used car based on roof type
    if condition.0 == Age::Used {
        if convertible == true {
            println!("Prepare a used car: {:?}, {}, Hard top, {} miles\n", transmission, color, condition.1); 
        } else {
            println!("Prepare a used car: {:?}, {}, Soft top, {} miles\n", transmission, color, condition.1);
        }

    } else {
        println!("Build a new car: {:?}, {}, Hard Top {}, {} miles\n", transmission, color, convertible, condition.1);
    }
             

    // Use the values of the input arguments
    // All new cars always have zero mileage
    let car = Car {
        color: String::from(&color),
        transmission: Transmission::from(transmission),
        convertible: convertible,
        mileage: condition,
    };

    car

}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
fn car_quality (miles: u32) -> (Age, u32) {

    let mut condition = Age::New;
    // Check milage and derive Age from it
    if miles > 0 {
        condition = Age::Used;
    }
    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    let quality: (Age, u32) = (condition, miles);
    // Return the completed tuple to the caller
    quality
}