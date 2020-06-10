use std::env;
use rand::{thread_rng, Rng};

mod parser;
use crate::parser::*;


pub struct Specifications{
    generate: bool,
    password_len: u32,
    charset: Vec<u8>,
}

impl Specifications{
    fn new() -> Specifications{
        return Specifications{
            generate : false,
            password_len: 30,
            charset: Vec::new(),
        };
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut specs = Specifications::new();
    let arguments: Vec<String> = env::args().collect();
    parser(&arguments , &mut specs);

    if specs.generate == true {

       let password: String = (0..specs.password_len)
            .map(|_|{
            let index: usize= rng.gen_range(0 , specs.charset.len());
            specs.charset[index] as char
                            })
            .collect();

        println!("{}" , password);
    }
}
