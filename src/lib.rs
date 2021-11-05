use array2d::Array2D;
use std::borrow::BorrowMut;

use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;

struct Index2d {
    row:u8,
    column:u8,
}

impl Index2d {
    fn new(row:u8, column:u8) -> Index2d{
        Index2d {
            row: row,
            column: column,
        }
    }

    fn next(&mut self) {
        self.column += 1;
        self.row += self.column / 9;
        self.column = self.column % 9;
    }

    fn prev(&mut self) {
        if self.column == 0 {
            self.row -= 1;
            self.column = 8;
        } else {
            self.column -= 1;
        }
    }

    fn to_xy(&self) -> (usize, usize){
        (self.row.into(), self.column.into())
    }

    fn to_iter_num(&self) -> u8 {
        self.row * 9 + self.column
    }
}

fn soduku_checker(soduku: &Array2D<i8>, index: &Index2d) -> bool {
    let tar = soduku[index.to_xy()];
    //Check the row have no identical numbers
    if soduku.row_iter(index.to_xy().0).filter(|x| (**x).abs() == tar.abs()).count() != 1 {
        return false;
    }

    //Check the column have no identical numbers
    if soduku.column_iter(index.to_xy().1).filter(|x| (**x).abs() == tar.abs()).count() != 1 {
        return false;
    }

    //Check no identical numbers on the 3*3 block
    let row_index = (index.to_xy().0 / 3) * 3;
    let column_index = (index.to_xy().1 / 3) * 3;
    let mut counter = 0;
    for i in row_index ..= row_index + 2 {
        for j in column_index ..= column_index + 2 {
            if soduku[(i, j)].abs() == tar.abs() {
                counter += 1;
                if counter >= 2 {
                    return false;
                }
            }
        }
    }

    true
}

fn soduku_trace_back(soduku: &mut Array2D<i8>, index: &mut Index2d) -> bool {
    loop {
        if soduku[index.to_xy()] <= -9 {    //value alright out of range
            soduku[index.to_xy()] = 0;
        }else if soduku[index.to_xy()] < 0 {
            break;
        }
        if index.to_iter_num() == 0 {
            // println!("No solution for this Soduku...");
            // process::exit(1);
            return false;
        }
        index.prev();
    }

    true
}

fn soduku_solver(soduku: &mut Array2D<i8>) -> bool{
    let mut index = Index2d::new(0,0);
    // let ten_millis = time::Duration::from_millis(1);
    while index.to_iter_num() < 81 {
        // println!("Current x-y is {:?}", index.to_xy());
        // print_2d_array(soduku);
        // println!("");
        // thread::sleep(ten_millis);
        let mut flag_going_back = false;
        if soduku[index.to_xy()] <= 0 {
            soduku[index.to_xy()] -= 1;
            while soduku[index.to_xy()] >= -9 {
                if soduku_checker(soduku, &index) {
                    break;
                }
                soduku[index.to_xy()] -= 1;
            }
            if soduku[index.to_xy()] < -9 {
                if soduku_trace_back(soduku, &mut index) {
                    flag_going_back = true;
                } else {
                    return false;
                }
            }
        }
        if !flag_going_back { index.next() };
    }

    true
}

fn quick_check(soduku: &Array2D<i8>) -> bool{
    let mut index = Index2d::new(0,0);
    while index.to_iter_num() < 81 {
        if soduku[index.to_xy()] > 0 {
            if !soduku_checker(soduku, &index) {
                return false;
            }
        }
        index.next();
    }

    true
}

fn print_2d_array(array2d: &Array2D<i8>) {
    for row_iter in array2d.rows_iter() {
        for element in row_iter {
            print!("{} ", element.abs());
        }
        println!();
    }
}

#[no_mangle]
pub extern "C" fn soduku_launcher(sudoku_array: &mut [i8; 82]) -> *mut [i8; 82] {
    let mut index = 0;
    let func = || {
        let tmp = index;
        index += 1;
        sudoku_array[tmp]
    };
    let mut array_2d = Array2D::filled_by_row_major(func,9,9);
    let mut result_array = [0_i8; 82];
    if quick_check(&array_2d){
        if soduku_solver(array_2d.borrow_mut()) {
            let mut result_vec = array_2d.elements_row_major_iter().
                into_iter().cloned().collect::<Vec<_>>();               //.into_iter().cloned() to obtain the value nor borrow in a tricky way
            result_vec.push(0_i8);
            result_vec = result_vec.iter().map(|x| x.abs()).collect();
            let mut counter = 0;
            for i in result_vec.iter() {
                result_array[counter] = *i;
                counter += 1;
            }
        } else {
            result_array[81] = 1;
        }
    }else {
        result_array[81] = 2;
    }
    let result_box = Box::new(result_array);
    Box::into_raw(result_box)
}

fn random_pos_generator() -> (usize, usize){
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..9) as usize, rng.gen_range(0..9) as usize)  //(0..9) is [0,8]
}

#[no_mangle]
pub extern "C" fn sudoku_puzzle_generator(difficulty: i32) -> *mut [i8; 82] {
    let mut num_of_unknows = 0;
    if difficulty == 0{
        num_of_unknows = 45;
    }else if difficulty == 1{
        num_of_unknows = 53;
    }else {
        num_of_unknows = 57;
    }
    let mut pivot: Vec<i8>= vec![1,2,3,4,5,6,7,8,9];

    pivot.shuffle(&mut thread_rng());

    let mut array_2d = Array2D::filled_with(0_i8, 9, 9);
    let pivot_pos: [(usize, usize); 9] = [(0,0), (3,1), (6,2), (1,3), (4,4), (7,5), (2,6), (5,7), (8,8)];
    for i in 0..=8 {
        array_2d[pivot_pos[i]] = pivot[i];      //Those value are the default value for the  Sudoku
    }                                           //After my test their distribution are randomly

    soduku_solver(array_2d.borrow_mut());

    while num_of_unknows > 0 {
        let temp = random_pos_generator();
        if array_2d[temp] != 0 {
            array_2d[temp] = 0;
            num_of_unknows -= 1;
        }
    }

    let mut result_array = [0_i8; 82];
    let mut result_vec = array_2d.elements_row_major_iter().into_iter().cloned().collect::<Vec<_>>();             
    result_vec = result_vec.iter().map(|x| x.abs()).collect();
    let mut counter = 0;
    for i in result_vec.iter() {
        result_array[counter] = *i;
        counter += 1;
    }
    let result_box = Box::new(result_array);
    Box::into_raw(result_box)
}

#[no_mangle]
pub extern "C" fn sudoku_check_answer(sudoku_array: &mut [i8; 82]) -> i8 {
    let mut index = 0;
    let func = || {
        let tmp = index;
        index += 1;
        sudoku_array[tmp]
    };
    let array_2d = Array2D::filled_by_row_major(func,9,9);

    let mut index = Index2d::new(0,0);
    while index.to_iter_num() < 81 {
        if array_2d[index.to_xy()] == 0 || !soduku_checker(&array_2d, &index){
            return 0_i8;
        }
        index.next();
    }

    1_i8
}
