#[derive(Debug)]
struct Salad {
    name: String,
    ingredients: Vec<String>,
    price: u8,
}

impl Salad {
    fn get_price_per_ingredient(&self) -> f32 {
        let ingredients_number: f32 = self.ingredients.len() as f32;
        let price: f32 = self.price as f32;

        price / ingredients_number
    }
}

fn main() {
    let vegan_salad = Salad {
        name: "Vegan".to_string(),
        ingredients: vec![
            "Lettuce".to_string(),
            "Tomato".to_string(),
            "Avocado".to_string(),
            "Onion".to_string(),
        ],
        price: 15,
    };

    println!("{:?}", vegan_salad);

    let price_per_ingredient = vegan_salad.get_price_per_ingredient();
    println!("{}", price_per_ingredient);
}
