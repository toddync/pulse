#include <unistd.h>
#include <stdio.h>

void _mlir_ciface_sleep(int seconds) {
    usleep(seconds * 1000000); // Convert seconds to microseconds
}

void _mlir_ciface_print_i32(int value) {
    printf("%d ", value);
}

void _mlir_ciface_println() {
    printf("\n");
}

void _mlir_ciface_print_chr(int c) {
    printf("%c", c);
}