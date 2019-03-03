from timeit import timeit
import os
from cffi import FFI

@timeit
def reverse_strings_ffi(data, lib, ffi):
    # Receive result pointer from foreign rust lib.
    # Of course it will compute result faster than python can. 
    # In this example raw result from rust is ready at 2 times faster then python's.
    result_ptr = lib.reverse_input(data, len(data))
    # But after we received the result pointer, we have to unpack the data behind it.
    # And here we loose even more time then "raw" computing.
    return get_str_list_from_ptr(result_ptr, len(data), ffi)

@timeit
def get_str_list_from_ptr(ptr, length, ffi):
    # It takes more time to unpack the computed result
    unpacked = ffi.unpack(ptr, length)
    return [ffi.string(x).decode() for x in unpacked]

@timeit
def main():
    path = 'words.txt'

    with open(path) as f:
        data = [x.strip('\n') for x in f.readlines()]

    base_dir = os.path.dirname(__file__)
    lib_name = "target/release/libstring_pipe.so"

    path = os.path.join(base_dir, lib_name)

    ffi = FFI()
    ffi.cdef(
        """
            int8_t **reverse_input(const char*[], size_t);
        """
    )
    lib_string_pipe = ffi.dlopen(path)
    input_arr = [ffi.new("char[]", x.encode()) for x in data]
    for _ in range(20):
        rev = reverse_strings_ffi(input_arr, lib_string_pipe, ffi)
        
    # CONCLUSION: There is no sense to work with rust foreign lib on python,
    #  when you have to continue the job with returned data.
    # Because cffi python package (in ABI mode) can`t unpack quickly complex data like list of strings
    #  (which are available through a raw pointer). 
    # Maybe there are more effective ways to get a data from foreign rust function but i did not find it.

if __name__ == '__main__':
    main()
