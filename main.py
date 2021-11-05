from cffi import FFI
from flask import Flask, request, render_template


ffi = FFI()
rust_func = ffi.dlopen("target/debug/librust_soduku.so")
ffi.cdef("""
    int8_t* soduku_launcher(int8_t*);
    int8_t* sudoku_puzzle_generator(int32_t);
    int8_t sudoku_check_answer(int8_t*);
""")

app = Flask(__name__)

def format_and_send(raw_data):
    int_list = list(map(int, raw_data.strip('[').strip(']').split(',')))

    c_array = ffi.new("int8_t[]", int_list)
    raw_result = ffi.cast("int8_t*",rust_func.soduku_launcher(c_array))

    raw_result = ffi.buffer(raw_result, 82)
    int_result = list(map(lambda x: int.from_bytes(x, "big"), raw_result))
    # return ','.join(map(str, int_result))
    return int_result

@app.route('/')
def home_page():
    return render_template('home_page.html')

@app.route('/solve_sudoku')
def solve_sudoku():
    return render_template('solve_sudoku.html')

@app.route('/play_sudoku')
def play_sudoku():
    return render_template('play_sudoku.html')

@app.route('/api/solve_sudoku', methods=['POST'])
def get_sudoku_msg():
    data = request.data.decode("utf-8")
    result = format_and_send(data)
    if result[81] == 1:             #Sudoku no solution...
        return "0", 202
    elif result[81] == 2:           #illegal Sudoku inputs...
        return "0", 203 
    return ','.join(map(str, result)), 200

@app.route('/api/play_sudoku', methods=['POST'])
def create_sudoku_msg():
    data = request.data.decode("utf-8")
    data_int = int(data)
    # c_int = ffi.new("int32_t", data_int)
    raw_result = ffi.cast("int8_t*", rust_func.sudoku_puzzle_generator(data_int))
    raw_result = ffi.buffer(raw_result, 82)
    int_result = list(map(lambda x: int.from_bytes(x, "big"), raw_result))
    return ','.join(map(str, int_result)), 200

@app.route('/api/check_sudoku', methods = ['POST'])
def check_sudoku_msg():
    data = request.data.decode("utf-8")
    int_list = list(map(int, data.strip('[').strip(']').split(',')))

    c_array = ffi.new("int8_t[]", int_list)
    bool_result = rust_func.sudoku_check_answer(c_array)
    # result = int.from_bytes(bool_result, "big")
    print("I'm send ", str(bool_result))
    return str(bool_result), 200


if __name__== '__main__':
    app.run(host = '0.0.0.0', port = 5000, debug=True)