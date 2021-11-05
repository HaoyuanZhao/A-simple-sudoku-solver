# Small-Sudoku-Tool
This is a web based application that can create and solve a sudoku puzzle using multiply programming languages.

# Development
Required Library:  
Python: cffi, flask  
Rust: array2d, rand, BorrowMut  

You need to build the Rust first, then run the main code.

    cargo build
    python3 main.py

# About the code
To achieve the optimal running speed, this project use Rsut as the mian programming language to implement the algorithm which generate/solve the sudoku puzzle.  
Possible outputs for solving a sudoku puzzle: 1. successfully solved 2. no solution for this Sudoku 3. User's illegal inputs (e.g.,two identical numbers in a same row/column/3*3 block)  
Possible outputs for generating a sudoku puzzle (for user to solve): 1.You get the correct answer 2. The answer is not correct...

# Future version
The generating Sudoku puzzle only consider the numbers of unknown to adjust the difficulty, which is not quite correct for a real Sudoku puzzle. It should also considering the difficulty in logic. For example, two Sudoku puzzle with same unknowns numbers, the difficulty can be dramatically different according the position and distribution of the numbers. Secondly, sometimes is obviously to see that the Sudoku puzzle is generating by "machine". Like the unusual sequence "1,2,3", or "6,7,8". This caused by the basic logic of my algorithm and need to improve in the further.

# License
MIT License

Copyright (c) 2021 HaoyuanZhao

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
