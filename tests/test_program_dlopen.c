#include <stdio.h>
#include <dlfcn.h>

typedef int (*add_func)(int, int);
typedef void (*print_hello_func)();
typedef int (*add_and_double_func)(int, int);
typedef void (*set_five_func)(int *);

int main() {
    void *handle = dlopen("./test_output/target/release/libmock_lib.so", RTLD_LAZY);
    if (!handle) {
        fprintf(stderr, "Failed to load library: %s\n", dlerror());
        return 1;
    }

    add_func add = (add_func)dlsym(handle, "add");
    if (!add) {
        fprintf(stderr, "Failed to find add function: %s\n", dlerror());
        return 1;
    }

    print_hello_func print_hello = (print_hello_func)dlsym(handle, "print_hello");
    if (!print_hello) {
        fprintf(stderr, "Failed to find print_hello function: %s\n", dlerror());
        return 1;
    }

    add_and_double_func add_and_double = (add_and_double_func)dlsym(handle, "add_and_double");
    if (!add_and_double) {
        fprintf(stderr, "Failed to find add_and_double function: %s\n", dlerror());
        return 1;
    }

    set_five_func set_five = (set_five_func)dlsym(handle, "set_five");
    if (!set_five) {
        fprintf(stderr, "Failed to find set_five function: %s\n", dlerror());
        return 1;
    }

    int result_add = add(5, 3);
    printf("dlopen: Testing add(5, 3) = %d\n", result_add);
    if (result_add != 8) {
        fprintf(stderr, "add(5, 3) failed: expected 8, got %d\n", result_add);
        return 1;
    }

    printf("dlopen: Calling print_hello:\n");
    print_hello();

    int result_double = add_and_double(5, 3);
    printf("dlopen: Testing add_and_double(5, 3) = %d\n", result_double);
    if (result_double != 16) {
        fprintf(stderr, "add_and_double(5, 3) failed: expected 16, got %d\n", result_double);
        return 1;
    }

    int value = 0;
    set_five(&value);
    printf("dlopen: Testing set_five(&value), value = %d\n", value);
    if (value != 5) {
        fprintf(stderr, "set_five failed: expected 5, got %d\n", value);
        return 1;
    }

    dlclose(handle);
    return 0;
}
