if [[ ! -d "build/linux" ]]; then
    mkdir build/linux
fi

g++ source/main.cpp \
    source/modules/math.cpp \
    -Iinclude -std=c++20 \
    -o build/linux/cppcalc