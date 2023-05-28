fn basic_structs() {
    #[derive(Debug)]
    struct Animal {
        name: String, // this is called 'field'
        specie: String,
        age: u8,
    }
    let name = String::from("Zeus");
    let specie = String::from("Perro");
    let age = 3;

    let dog = Animal { name, specie, age };
    let dog2 = Animal {
        name: String::from("Mocca"),
        ..dog
    };

    println!("La struct es {:?}", dog2);
    println!(
        "{} es de especie {} y tiene {} años.",
        dog2.name, dog2.specie, dog2.age
    );
}

fn tuple_structs() {
    #[derive(Debug)]
    struct ColorRGB(u8, u8, u8, String);

    let red = ColorRGB(255, 0, 0, String::from("red"));
    println!("El color es {:?}", red);

    // We need to type the tuple struct when destructuring
    let ColorRGB(red_r, red_g, red_b, red_name) = red;
    println!("El color \"{red_name}\" formado por {red_r} {red_g} {red_b}");
}

fn main() {
    basic_structs();
    tuple_structs();
}
