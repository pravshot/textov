use textov::Textov;

fn main() {
    let start = std::time::Instant::now();
    let tev = Textov::new("datasets/long-example.txt".to_string());
    tev.print_all();
    println!("{}", tev.generate_sentence());
    let end = std::time::Instant::now();
    println!("{}", end.duration_since(start).as_millis());
}
// 20.92 without concurrency
// 17.92 with concurrency