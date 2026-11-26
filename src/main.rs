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

    let first_number: f32;
    let second_number: f32;
    let result: f32;

    let action_choice: i8;

    let factorial: bool;
    // End //

    // Arguments check //
    if argc < 3 || argc > 3 
    {
        println!("Usage: \n- cargo run -- -fac <number>\n- cargo run -- <number> <number>\n");
        return;
    }
    factorial = argv[1] == "-fac";
    // End //

    if factorial
    {
        let number = match argv[2].parse()
        {
            Ok(value) => value,
            Err(error) => {
                println!("Error: {}, Hm.. Second Number isn't a number, are you kidding me?\n", error);
                return;
            }
        };
        let result: u64 = highmaths::maths_factorial(number);
        println!("Result: {}", result);

        return;
    }
    else
    {
        first_number = match argv[1].parse()
        {
            Ok(value) => value,
            Err(error) => {
                println!("Error: {}, Hm.. First Number isn't a number, are you kidding me?\n", error);
                return;
            }
        };
        second_number = match argv[2].parse()
        {
            Ok(value) => value,
            Err(error) => {
                println!("Error: {}, Hm.. Second Number isn't a number, are you kidding me?\n", error);
                return;
            }
        };
    }

    // Main Block //
    let mut buff = String::new();

    println!(">>  ");
    io::stdin()
        .read_line(&mut buff)
        .expect("Impossible to read line!");

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