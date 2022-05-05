use textov::Textov;

fn main() {
    let tev = Textov::new("datasets/goldilocks.txt".to_string());
    println!("{}", tev.generate_text(5));
}
