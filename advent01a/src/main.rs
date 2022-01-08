use std::path::Path;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use itertools::zip;

extern crate itertools;

fn main()-> io::Result<()>{

    let file = File::open(Path::new("input.txt")).expect("File not found.");
    let reader = BufReader::new(file);

    let numbers:Vec<i32> = reader.lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap()).collect();

    let z = zip(numbers.iter().cloned(),numbers[1..].iter().cloned());
    let _count:i32= z.map(|(a, b)| (a-b<0) as i32).sum();
    println!("There were {} increase measurements from the sub.",_count);

    Ok(())
}
