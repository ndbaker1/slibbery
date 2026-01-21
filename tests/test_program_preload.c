#include <stdio.h>

// Declare functions (will be resolved by LD_PRELOAD)
extern int add(int a, int b);
extern void print_hello();
extern int add_and_double(int a, int b);
extern void set_five(int *ptr);

int main() {
    int result_add = add(5, 3);
    printf("LD_PRELOAD: Testing add(5, 3) = %d\n", result_add);
    if (result_add != 8) {
        fprintf(stderr, "add(5, 3) failed: expected 8, got %d\n", result_add);
        return 1;
    }

    printf("LD_PRELOAD: Calling print_hello:\n");
    print_hello();

    int result_double = add_and_double(5, 3);
    printf("LD_PRELOAD: Testing add_and_double(5, 3) = %d\n", result_double);
    if (result_double != 16) {
        fprintf(stderr, "add_and_double(5, 3) failed: expected 16, got %d\n", result_double);
        return 1;
    }

    int value = 0;
    set_five(&value);
    printf("LD_PRELOAD: Testing set_five(&value), value = %d\n", value);
    if (value != 5) {
        fprintf(stderr, "set_five failed: expected 5, got %d\n", value);
        return 1;
    }

    return 0;
}
