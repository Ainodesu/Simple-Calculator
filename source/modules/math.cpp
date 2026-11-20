#include "math.hpp"
#include <iostream>

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
    if (y == 0.0f || x == 0.0f)
    {
        std::cout << "Division by 0 is prohibited!" << std::endl;
        return 0;
    }
    return x / y;
}
float Math::Multiply(float x, float y)
{
    if (y == 0.0f || x == 0.0f)
    {
        std::cout << "Multiplying by 0 is prohibited!" << std::endl;
        return 0;
    }
    return x * y;
}