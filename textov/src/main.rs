use textov::Textov;

fn main() {
    let tev = Textov::new("datasets/goldilocks.txt".to_string());
    tev.print_all();
    println!("{}", tev.generate_text(4));

}