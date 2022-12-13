struct Counter {
    length: usize,
    count: usize,
}

// An Iterator has a method, next, which when called returns an Option<Item>. The next method will return Some(Item) as long as there are elements. After they've all been exhausted, it will return None to indicate that iteration is finished.

impl Counter {
    fn new(length: usize) -> Counter {
        Counter { count: 0, length }
    }
}

// The next() method is the only required method that we should define. Inside its body, we increment our count by one at every call (which is why we started at zero). Then we check to see if we've finished counting or not. We use the Some(value) variant of the Option type to express that iteration is still yielding results and the None variant to express that iteration should stop.
impl Iterator for Counter {
    type Item = usize;

    // Self::Item means that the Item type will be the type returned from the iterator inside the for loop block
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    for number in Counter::new(10) {
        println!("{}", number);
    }

    let sum_until_10: usize = Counter::new(10).sum();
    assert_eq!(sum_until_10, 55);

    let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    assert_eq!(powers_of_2, vec![2, 4, 8, 16, 32, 64, 128, 256]);
}
