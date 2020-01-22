fn main() {
    let name = "Rust";
    println!("{}", {name}.to_owned() + " " + "lang")
}