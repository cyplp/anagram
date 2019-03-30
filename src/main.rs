fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let dict = args[0].clone();
    let searching = args[1].clone();

    println!("searching {} in {}", searching, dict);
}
