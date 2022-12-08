fn bucle_loop() {
    let mut counter = 1;
    // stop_loop is set when loop stops
    println!("\nLoop function:");
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("Break the loop at counter = {}.", stop_loop);
}
fn bucle_while() {
    let mut counter_while = 1;
    println!("\nWhile function:");
    while counter_while < 5 {
        println!("We loop a while...");
        counter_while = counter_while + 1;
    }
}
fn bucle_for() {
    let big_birds = ["ostrich", "peacock", "stork"];
    println!("\nFor function:");
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }

    // Iterator: from 0 to 5
    for number in 0..5 {
        println!("{}", number);
    }
}

fn main() {
    bucle_loop();
    bucle_while();
    bucle_for();
}
