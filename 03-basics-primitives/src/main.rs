fn print_type_of<T>(_: &T) {
    println!("The type is: {}", std::any::type_name::<T>())
}

fn main() {
    // ? PRIMITIVES

    // ! Scalar types
    let boolean: bool = true;
    let character: char = '🦀';
    let integer: u8 = 17;
    let float: f32 = 14.4;

    println!("Our datas: {boolean}, {character}, {integer}, {float}");
    print_type_of(&character);

    // ! Compound types

    // ? Arrays
    let fruits = ["Apple", "Banana", "Orange"];
    println!("Tengo {} frutas", fruits.len());

    // Destructuring: array
    let [apple, banana, _] = fruits;
    println!("Come {apple} y {banana}");

    // Discarting unused destructured
    let [.., orange] = fruits;
    println!("Por último {orange}");

    // ? Tuples
    // Inferred types
    let _product = ("Computer", 550, true);

    // Declared types
    let product: (&str, i32, bool) = ("Computer", 550, true);

    // Destructuring: tuple
    let (_, product_price, _) = product;
    println!("El producto cuesta {product_price}");

    // Discarting unused destructured
    let (product_name, ..) = product;
    println!("El producto es {product_name}");

    // Discarting unused destructured
    let (.., has_stock) = product;
    println!("El producto {product_name} tiene stock? {has_stock}");
}
