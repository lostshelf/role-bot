use std::env;

fn main() {
    let _token = match env::var("TOKEN") {
        Ok(t) => t,
        Err(e) => panic!("Couldn't get token from environment: {}", e),
    };
}