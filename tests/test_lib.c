#include <stdio.h>

int add(int a, int b) {
    return a + b;
}

void print_hello() {
    printf("Hello from original library!\n");
}

int add_and_double(int a, int b) {
    int sum = add(a, b);
    return sum * 2;
}

void set_five(int *ptr) {
    *ptr = 5;
}
