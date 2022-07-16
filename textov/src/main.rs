use std::time::Instant;
use textov::Textov;

fn main() {
    // start time
    let start = Instant::now();
    // Create a new Textov instance with the filepath to the text file
    // this will be the dataset that will be used to generate sentences
    let tev = Textov::new("datasets/Book1.txt".to_string());
    // We can use tev.generate_sentence() to generate a sentence or 
    // tev.generate_text(usize) to generate multiple sentences and combine them together
    println!("{}", tev.generate_text(5));
    // end time
    let end = Instant::now();
    println!("{:?}", end.duration_since(start));

    let start = Instant::now();
    let tev_without_concurrency = Textov::new_without_concurrency("datasets/Book1.txt".to_string());
    println!("{}", tev_without_concurrency.generate_text(5));
    let end = Instant::now();
    println!("{:?}", end.duration_since(start));

    // As you can tell by running the program, concurrency is significantly faster.
}
