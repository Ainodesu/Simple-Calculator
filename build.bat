@echo off

if not exist build\win (
    mkdir build\win
)

g++ source\main.cpp ^
    source\modules\math.cpp ^
    -Iinclude\ -std=c++20 ^
    -o build\win\cppcalc.exe