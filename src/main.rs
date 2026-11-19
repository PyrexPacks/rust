use std::io;
use rand::prelude::*;

fn main() {


    let random = rand::rng().random_range(1..=100);
    println!("A titkos szam = {random}");

    println!("Irj be egy szamot!");

    let mut szam = String::new();

    io::stdin()
        .read_line(&mut szam)
        .expect("ez nem ment ocsipok");

    println!("ez a szam {szam}")
}
