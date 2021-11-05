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
