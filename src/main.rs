#[path = "modules/maths.rs"]
mod maths;
use std::env;
use std::io;

fn main()
{
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argc < 3
    {
        println!("Usage: {} <num> <num>\n", argv[0]);
        return;
    }

    let first_number: f32 = argv[1].parse::<f32>().unwrap();
    let second_number: f32 = argv[2].parse::<f32>().unwrap();
    let result: f32;

    let choice: i8;

    // Main Block //
    let mut buff = String::new();

    println!("Choose an action(Add, Sub, Div, Mul): ");
    io::stdin()
        .read_line(&mut buff)
        .expect("Impossible to read line!");

    if buff.trim() == "Add"
    {
        choice = 0;
    }
    else if buff.trim() == "Sub"
    {
        choice = 1;
    }
    else if buff.trim() == "Div"
    {
        choice = 2;
    }
    else if buff.trim() == "Mul"
    {
        choice = 3;
    }
    else
    {
        println!("Unknown Action!\n");
        return;
    }

    match choice
    {
        0 => result = maths::maths_add(first_number, second_number),
        1 => result = maths::maths_substract(first_number, second_number),
        2 => result = maths::maths_divide(first_number, second_number),
        3 => result = maths::maths_multiply(first_number, second_number),
        _ => unreachable!()
    }

    println!("Result: {}", result);
}