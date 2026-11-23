pub fn maths_pow(x: f32, y: f32) -> f32
{
    x.powf(y)
}
pub fn maths_factorial(n: u64) -> u64
{
    let mut result: u64 = 1;

    for i in 1..=n
    {
        result *= i;
    }

    return result;
}