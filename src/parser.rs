use crate::Specifications;

//function to parse command line arguments
//takes reference to a vector of strings and a mutable reference to a struct containing all the
//specifications based on the command line arguments


pub fn parser(args: &Vec<String>, specs: &mut Specifications){
    let _y: &str;
    let usage = "<USAGE>\nI don't care about the order of the arguments.\nHere's list of arguments that you can use:\n\n1. -h , -help   --> To get this same boring Usage message :-)\n2. -g , -generate --> to generate a random passwoed (without it, I won't generate anything).\n3. -sp , -special --> To have special characters in your random password.\n4. Any gibberish to get this help message again ;-).\n\n\nBtw I don't save your passwords! Would that be a good feature?\n\n";
    //converting the string to vector array
    let charset_with_all: Vec<u8> = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789*&^%$#@!~".to_vec();
    
    let charset_with_alphabets: Vec<u8>  = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_vec();
    
    for i in &args[1..]{
        match i.as_str(){
            "-h" | "-help" => {
                    // print <USAGE>
                    println!("{}", usage);
                    break;
            },
            "-g" | "-generate" => {
                    specs.generate= true;
                    specs.charset = charset_with_alphabets.clone();
            },
            "-sp" | "-special" => {
                    specs.charset = charset_with_all.clone();
            },
            y  => {
                    match y.parse::<u32>() {
                        Ok(n) => {
                            specs.password_len = n;
                        }
                        Err(_) => {
                            println!("{}", usage);
                            break;
                        }
                    }   
            },
        }
    }
}
