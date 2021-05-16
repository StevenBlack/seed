use std::env;

fn main() {
    // command line arguments
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
