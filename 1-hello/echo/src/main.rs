use std::io::{stdin, BufReader, BufRead};

fn main() {
    let lines = BufReader::new(stdin()).lines();
    for line in lines {
        println!("{}", line.unwrap());
    }
}
