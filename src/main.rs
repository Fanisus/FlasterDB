// Imports //
use rand::Rng;
use std::io;
use std::time;

// MAIN //
fn main() {
    loop {
        let mut input = String::new();
        // io::stdin().read_line(&mut input).unwrap();
        // println!("{input}");
        let mut input = "select * from".split(" ");
        let input: Vec<&str> = input.collect();
        // let mut first_arg = input.get(0).unwrap().trim().to_owned();
        print!("{:?}", input);
        if (input.get(0).unwrap().trim().eq_ignore_ascii_case("SELECT")) {
            println!("{}", input.get(0).unwrap().trim());
            if (input.get(1).unwrap().trim().eq_ignore_ascii_case("*")) {
                println!("{}", input.get(1).unwrap().trim());

                if (input.get(2).unwrap().trim().eq_ignore_ascii_case("FROM")) {
                    println!("{}", input.get(2).unwrap().trim());
                    print!();
                    println!("Ok")
                }
            }
        } else {
            println!(">>>>> {:?}", input)
        }
    }
}
