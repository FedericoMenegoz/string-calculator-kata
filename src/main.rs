use colored::*;
use std::process::exit;
use string_calculator_lib::add;

pub fn main() {
    let mut args = std::env::args();
    let program_name = args.next().expect("Program name should always be present.");

    if args.len() != 1 {
        println!(
            "\n{}: {program_name} <string argument>",
            "Usage".bold().green()
        );

        let help_msg = "
    - Enter numbers separated by a comma 
    - To specifiy a custom delimiter start with \"//[delimiter]\\n\"
    - No negatives number allowed
    - Values greater than 1000 will be ignored\n"
            .truecolor(128, 128, 128);

        println!("{}{}", "Help:".bold().blue(), help_msg);

        exit(1);
    }

    let input = args
        .next()
        .expect("Already checked number of arguments.")
        .replace("\\n", "\n");

    match add(&input) {
        Ok(val) => println!("The sum is: {val}"),
        Err(s) => panic!("{}", s.red().bold()),
    }
}
