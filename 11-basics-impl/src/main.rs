#[derive(Debug)]
struct Salad {
    name: String,
    ingredients: Vec<String>,
    price: u8,
}

impl Salad {
    // When using 'self', we can use the method from any struct instance
    fn get_price_per_ingredient(&self) -> f32 {
        let ingredients_number: f32 = self.ingredients.len() as f32;
        let price: f32 = self.price as f32;

        price / ingredients_number
    }

    // When not using 'self', we can use the method from the Struct
    fn eat() {
        println!("Salad eaten!");
    }

    fn set_price(&mut self, new_price: u8) {
        self.price = new_price;
    }
}

fn main() {
    let mut vegan_salad = Salad {
        name: "Vegan".to_string(),
        ingredients: vec![
            "Lettuce".to_string(),
            "Tomato".to_string(),
            "Avocado".to_string(),
            "Onion".to_string(),
        ],
        price: 15,
    };

    println!("Salad data: {:?}", vegan_salad);

    // Using the impl method from the instance
    let price_per_ingredient = vegan_salad.get_price_per_ingredient();
    println!("The average price per ingredient is: {price_per_ingredient}");

    // Modify the instance data
    vegan_salad.set_price(18);
    println!("The new price is: {}", vegan_salad.price);

    // Using the impl method from the struct
    Salad::eat();
}
