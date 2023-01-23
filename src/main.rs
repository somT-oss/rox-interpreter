use std::fs::File;
use std::io::{Error, Read, stdin};
use std::env;

fn read_file(file_name: String) -> Result<(), Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{:?}", contents);
    Ok(())
}

fn run_prompt() {
    let boolean = true;

    while boolean {
        let mut input_string = String::new();
        println!(">> ");
        stdin().read_line(&mut input_string).ok().expect("Could not read input");
        
        if input_string.trim().ends_with(";") {
            continue;
        } else {
            break;
        }
    }
}


fn main() {
    // if CLI arguments greater than one, output a string that shows the extra CLI arguments for ROX
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rox [script]")
    } else if args.len() == 2 { 
        read_file(args[1].clone()).unwrap();  
    } else {
        run_prompt();
    }
}   




