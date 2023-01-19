// This is my solution for the coding exercise

use rand::Rng;

fn main(){

// create car array with available colors
let mut colors = ["blue", "green", "red", "silver"];

// cast length of colors array to an integer
let colors_len = colors.len() as u32;

// We have orders for three new cars!
// We'll declare a mutable car variable and reuse it for all the cars
    // Create a new random configuration for each car
    // example: [2, 1, 0, 3000] = [color, transmission, convertible, mileage]
    // input: [color, mileage] (transmission, convertible) are fixed


    // Create new cars with random specs based on user input

    //get user input from command line
    let mut input = String::new();
    println!("How many cars do you want to build?");

    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut input: u32 = input.trim().parse().expect("Please provide the amount of cars (Integer)!");


    let mut configuration = random_configuration(colors_len, 10000);

    let mut color = String::from(colors[configuration.0 as usize]);
    let mut engine = configuration.1;
    let mut convertible = configuration.2;
    let mut age = car_quality(configuration.3);

    while input > 0 {
        let car = car_factory(color, engine, convertible, age);
        println!("Car is ready!");
        configuration = random_configuration(colors_len, 10000);
        color = String::from(colors[configuration.0 as usize]);
        engine = configuration.1;
        convertible = configuration.2;
        age = car_quality(configuration.3);
        input -= 1;
    }

    
}

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    transmission: Transmission,
    convertible: Convertible,
    mileage: Age,
}

type Age = (Condition, u32);

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Condition {
    New,
    Used,
}

#[derive(PartialEq, Debug)]
enum Convertible {
    HardTop,
    SoftTop,
}

// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: Convertible, condition: Age) -> Car {

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
fn car_quality (miles: u32) -> (Condition, u32) {

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

fn random_configuration(max_color: u32, max_milage: u32) -> (u32, Transmission, Convertible, u32) {
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