use std::io;
use std::io::Error;
mod handler;

fn read_one_line_from_std_in() -> Result<String, Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(String::from(input.trim())),
        Err(error) => {
            eprintln!("error: {}", error);
            Err(error)
        }
    }
}

fn main() {
    // read input
    let input = read_one_line_from_std_in().expect("Error in reading input!");

    // do some fancy processing
    let output = handler::process(input);
    //write output
    print!("{}", output);
}




