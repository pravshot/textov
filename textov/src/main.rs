use textov::Textov;

fn main() {
    // Create a new Textov instance with the filepath to the text file
    // this will be the dataset that will be used to generate sentences
    let tev = Textov::new("datasets/goldilocks.txt".to_string());
    // We can use tev.generate_sentence() to generate a sentence or 
    // tev.generate_text(usize) to generate multiple sentences and combine them together
    println!("{}", tev.generate_text(5));
}
