#include "math.hpp"
#include <iostream>
#include <cstring>

int main(int argc, char* argv[])
{
    if ( argc < 3 )
    {
        std::cout << "Usage: " << argv[0] << " <num> <num>" << std::endl;
        return -1;
    }
    // Program Block //
    /* Variables and more */
    enum Actions
    {
        Add,
        Sub,
        Div,
        Mul,
        Pow
    };

    float first_num = std::stof(argv[1]);
    float second_num = std::stof(argv[2]);
    float result;

    int choice = 0;
    /* Main block */
    std::string input;

    std::cout << "Choose an action(Add, Sub, Div, Mul, Pow): " << std::flush;
    std::getline(std::cin, input);


    if (std::strcmp(input.c_str(), "Add") == 0)
    {
        choice = Add;
    }
    else if (std::strcmp(input.c_str(), "Sub") == 0)
    {
        choice = Sub;
    }
    else if (std::strcmp(input.c_str(), "Div") == 0)
    {
        choice = Div;
    }
    else if (std::strcmp(input.c_str(), "Mul") == 0)
    {
        choice = Mul;
    }
    else if (std::strcmp(input.c_str(), "Pow") == 0)
    {
        choice = Pow;
    }
    else
    {
        std::cout << "Unknown Action!" << std::endl;
        return -1;
    }

    switch (choice)
    {
        case 0:
            result = Math::Add(first_num, second_num);
            break;
        case 1:
            result = Math::Substract(first_num, second_num);
            break;
        case 2:
            result = Math::Divide(first_num, second_num);
            break;
        case 3:
            result = Math::Multiply(first_num, second_num);
            break;
        case 4:
            result = Math::Pow(first_num, second_num);
            break;
        default:
            break;
    }

    std::cout << "Result: " << result << std::endl;
    return 0;
    // End //
}