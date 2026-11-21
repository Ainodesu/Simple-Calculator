#include "math.hpp"
#include <iostream>
#include <cmath>

float Math::Add(float x, float y)
{
    return x + y;
}
float Math::Substract(float x, float y)
{
    return x - y;
}
float Math::Divide(float x, float y)
{
    if (y == 0.0f)
    {
        std::cout << "Division by 0 is prohibited!" << std::endl;
        return 0;
    }
    return x / y;
}
float Math::Multiply(float x, float y)
{
    return x * y;
}
double Math::Pow(double x, double y)
{
    return std::pow(x, y);
}