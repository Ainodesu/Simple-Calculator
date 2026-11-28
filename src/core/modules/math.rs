// Simple Functions
pub fn math_addition(x: f32, y: f32) -> f32
{
    x + y
}
pub fn math_substract(x: f32, y: f32) -> f32
{
    x - y
}
pub fn math_divide(num: f32, denom: f32) -> f32
{
    if denom == 0.0
    {
        println!("Division by 0 is prohibited!\n");
        return 0.0;
    }

    num / denom
}
pub fn math_multiply(x: f32, y: f32) -> f32
{
    x * y 
}

// High Functions
pub fn math_power(x: f32, y: f32) -> f32
{
    x.powf(y)
}
pub fn math_factorial(n: u64) -> u64
{
    let mut result: u64 = 1;

    for i in 1..=n
    {
        result *= i;
    }

    result
}
pub fn math_sqrt(n: f32) -> f32
{
    if n <= 0.0
    {
        println!("Error: Square root of a negative number is prohibited!\n");
        return 0.0;
    }

    n.sqrt()
}