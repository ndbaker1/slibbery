#ifndef TEST_UNKNOWN_LIB_H
#define TEST_UNKNOWN_LIB_H

// Known functions (will have stubs generated)
int test_add(int a, int b);

// Unknown functions (will be accessed via RTLD_GLOBAL)
// int test_subtract(int a, int b);
// int test_multiply(int a, int b);
// int test_divide(int a, int b);

#endif