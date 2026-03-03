mod utils;

fn main() {
    println!("Hello, world!");
    const CODE_LENGTH: usize = 8;
    
    let code = utils::generate_code(CODE_LENGTH);

    println!("Your code is {}", code);
}
