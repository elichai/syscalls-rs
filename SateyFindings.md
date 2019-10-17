# Safety Improvements from rust rewrite.

1. Using `open(2)` with O_TMPFILE/O_CREAT but not supplying a mode will result in UB in musl (calling va_list functions where there's nothing). but in glibc it will try to give you a good default or return error.
