#[path = "modules/maths.rs"]
mod maths;
#[path = "modules/highmaths.rs"]
mod highmaths;

use std::env;
use std::io;

fn main()
{
    // Variables //
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    let first_number;
    let second_number;
    let result: f32;

    let action_choice: i8;

    let factorial: bool;
    // End //

    // Arguments check //
    if argc < 3
    {
        println!("Usage: {} -- <num> <num>\n or\n Usage: {} -- -flag <num>\n", argv[0], argv[0]);
        return;
    }
    factorial = argv[1] == "-fac";
    // End //

    if factorial
    {
        let number = argv[2].parse::<u64>().unwrap();
        let result: u64 = highmaths::maths_factorial(number);
        println!("Result: {}", result);

        return;
    }
    else
    {
        first_number = argv[1].parse::<f32>().unwrap();
        second_number = argv[2].parse::<f32>().unwrap();
    }

    // Main Block //
    let mut buff = String::new();
    if !factorial // if variable "factorial" is false, then do code in {} 
    {
        println!("Choose an action(Add, Sub, Div, Mul, Pow): ");
        io::stdin()
            .read_line(&mut buff)
            .expect("Impossible to read line!");
    }

    match buff.as_str().trim()
    {
        "Add" => action_choice = 0,
        "Sub" => action_choice = 1,
        "Div" => action_choice = 2,
        "Mul" => action_choice = 3,
        "Pow" => action_choice = 4,
        _ => { println!("Unknown Action!\n"); return; }
    }

    match action_choice
    {
        0 => result = maths::maths_add(first_number, second_number),
        1 => result = maths::maths_substract(first_number, second_number),
        2 => result = maths::maths_divide(first_number, second_number),
        3 => result = maths::maths_multiply(first_number, second_number),
        4 => result = highmaths::maths_pow(first_number, second_number),
        _ => unreachable!()
    }

    println!("Result: {}", result);
}