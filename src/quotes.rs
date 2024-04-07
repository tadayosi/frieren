use rand::prelude::SliceRandom;

const QUOTES: [&str; 3] = [
    "We only traveled together for a mere ten years...",
    "Yes, Himmel would say so.",
    "You are in front of a mage who has lived for more than one thousand years.",
];

pub fn quote() {
    let mut rnd = rand::thread_rng();
    println!("❄️  > {}", QUOTES.choose(&mut rnd).unwrap());
}
