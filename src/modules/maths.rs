pub fn maths_add(x: f32, y: f32) -> f32
{
    x + y
}
pub fn maths_substract(x: f32, y: f32) -> f32
{
    x - y
}
pub fn maths_divide(x: f32, y: f32) -> f32
{
    if x == 0.0 || y == 0.0
    {
        println!("Division by 0 is prohibited!\n");
        return 0.0;
    }

    x / y
}
pub fn maths_multiply(x: f32, y: f32) -> f32
{
    if x == 0.0 || y == 0.0
    {
        println!("Multiplying by 0 is prohibited!\n");
        return 0.0;
    }
    
    x * y
}