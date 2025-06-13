use rand::seq::IndexedRandom;

fn main() {
    println!("Hello, world!");
    let mut sentence = generate_random_words();
    for _ in 1..101 {
        println!("{}", sentence);
        sentence = generate_random_words();
    }
}

fn generate_random_words() -> String {
    let mut sentence = String::new();
    let what: Vec<&str> = vec!["watches tiktok", "plays mario golf", "watches ytp"];
    let who: Vec<&str> = vec!["tryh4rd", "instagram user", "rivolux", "beluga", "ooga booga"];
    let wher: Vec<&str> = vec!["in their house", "in the white house", "on discord", "in a javascript webgl container that itself is running in firefox on a web vm which itself is running on a kvm windows vm"];
    let why: Vec<&str> = vec!["because they like it", "because they want to do it", "because it's fun", "because it's addicting af", "because they procrastinate and never want to port their 2000 line c++ code into rust", "because it's their only choice"];
    let how: Vec<&str> = vec!["with their phone", "with a metal spatula", "with a slinky", "with a 20 year old pentium machine"];
    let when: Vec<&str> = vec!["at 2 am", "at 0:00 UTC", "at 9:00PM JST", "at 2PM", "while programming html"]; 

    match who.choose(&mut rand::rng()) {
        Some(string) => sentence.push_str(&string),
        None    => println!("failed"),
    }

    sentence.push_str(" ");

    match what.choose(&mut rand::rng()) {
        Some(string) => sentence.push_str(&string),
        None    => println!("failed"),
    }

    sentence.push_str(" ");

    match wher.choose(&mut rand::rng()) {
        Some(string) => sentence.push_str(&string),
        None    => println!("failed"),
    }

    sentence.push_str(" ");

    match how.choose(&mut rand::rng()) {
        Some(string) => sentence.push_str(&string),
        None    => println!("failed"),
    }

    sentence.push_str(" ");

    match when.choose(&mut rand::rng()) {
        Some(string) => sentence.push_str(&string),
        None    => println!("failed"),
    }

    sentence.push_str(" ");

    match why.choose(&mut rand::rng()) {
        Some(string) => sentence.push_str(&string),
        None    => println!("failed"),
    }

    return sentence;
}
