#[path = "core/modules/math.rs"]
mod math;

use std::env;
use std::io;
use std::io::Write;

fn main()
{
    // Variables //
    let version: &str = "v0.1.0-beta, November of 28 2026 y";

    let arguments: Vec<String> = env::args().collect();
    let arguments_length: usize = arguments.len();
    // End //

    // Arguments check //
    if arguments_length <= 1 || arguments_length > 3 
    {
        println!("Usage: \n- cargo run -- -fac <number>\n- cargo run -- <number> <number>\n");
        return;
    }
    if arguments[1].parse::<f32>().is_err()
    {
        match arguments[1].as_str()
        {
            "-help" => { println!("Actions:\n Add - x + y,\n Sub - x - y,\n Div - x / y,\n Mul - x * y,\n Pow - x ^ y,\n\n -fac <n> - n!,\n -sqrt <n> - a*a...n"); return; },
            "-version" => { println!("{0}", version); return; },
            "-fac" => 
            {
                let number = match arguments[2].parse()
                {
                    Ok(value) => value,
                    Err(error) => {
                    println!("Error is : \"{}\"! Second number is a something different, but not a number, isn't it?\n", error);
                    return;
                    }
                };
                let result: u64 = math::math_factorial(number);
                println!("Result is: {}", result);

                return;
            },
            "-sqrt" =>
            {
                let number: f32 = match arguments[2].parse()
                {
                    Ok(value) => value,
                    Err(error) => {
                    println!("Error is : \"{}\"! Second number is a something different, but not a number, isn't it?\n", error);
                    return;
                    }
                };
                let result: f32 = math::math_sqrt(number);
                println!("Result is: {}", result);

                return;
            },
            _ => { println!("Unknown Flag!\n"); return; }
        }
    }
    // End //
    let first_number: f32 = match arguments[1].parse()
    {
        Ok(value) => value,
        Err(error) => {
            println!("Error is: \"{}\"! First number is a something different, but not a number, isn't it?\n", error);
            return;
        }
    };
    let second_number: f32 = match arguments[2].parse()
    {
        Ok(value) => value,
        Err(error) => {
            println!("Error is: \"{}\"! Second number is a something different, but not a number, isn't it?\n", error);
            return;
        }
    };

    // Main Block //
    /* Input */
    let mut buff = String::new();

    print!(">> ");
    io::stdout().flush()
        .expect("Couldn't flush!");
    io::stdin()
        .read_line(&mut buff)
        .expect("Couldn't read a line!");
    /* END */

    let action_choice: i8 = match buff.as_str().trim()
    {
        "Add" => 0,
        "Sub" => 1,
        "Div" => 2,
        "Mul" => 3,
        "Pow" => 4,
        "Fac" | "-fac" => { println!("Usage: \n- cargo run -- -fac <number>\n"); return; },
        "Sqrt" | "-sqrt" => { println!("Usage: \n- cargo run -- -sqrt <number>\n"); return; },
        _ => { println!("Unknown Action!\n"); return; }
    };

    let result: f32 = match action_choice
    {
        0 => math::math_addition(first_number, second_number),
        1 => math::math_substract(first_number, second_number),
        2 => math::math_divide(first_number, second_number),
        3 => math::math_multiply(first_number, second_number),
        4 => math::math_power(first_number, second_number),
        _ => unreachable!()
    };

    println!("Result is: {}", result);
}