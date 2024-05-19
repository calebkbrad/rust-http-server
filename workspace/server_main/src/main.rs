use std::env;

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
}
