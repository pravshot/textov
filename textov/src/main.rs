use textov::Textov;

fn main() {
    let tev = Textov::new("datasets/green-eggs-and-ham.txt".to_string());
    tev.print_all();
    println!("{}", tev.generate_text(5));

}