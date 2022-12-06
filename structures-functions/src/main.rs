#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    mileage: (Age, u32),
}
#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_quality(miles: u32) -> (Age, u32) {
    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    // let quality = (Age, u32) = todo!("Set the `Age` value to \"New\", set the mileage using the `miles` input argument");
    if miles == 0 {
        return (Age::New, 0);
    }

    (Age::Used, miles)
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - "age" field calls "car_quality" function with "miles" input argument
    Car {
        color: color,
        motor: motor,
        roof: roof,
        mileage: car_quality(miles),
    }
}

fn main() {
    // Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Declare the car type and initial values
    let mut car: Car = Car {
        color: "silver".to_string(),
        motor: Transmission::Manual,
        roof: true,
        mileage: car_quality(0),
    };

    println!(
        "Initial card: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.mileage.0, car.roof, car.motor, car.color, car.mileage.1
    );

    let mut engine = Transmission::Manual;

    // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.mileage.0, car.roof, car.motor, car.color, car.mileage.1
    );

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[2]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.mileage.0, car.roof, car.motor, car.color, car.mileage.1
    );

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[1]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.mileage.0, car.roof, car.motor, car.color, car.mileage.1
    );
}
