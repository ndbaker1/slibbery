.PHONY: all clean test e2e

all:
	cargo build --release

clean:
	cargo clean
	rm -rf tests/test_output
	rm -f tests/libtest.so
	rm -f tests/test_program_dlopen tests/test_program_preload

test:
	cargo test

e2e:
	cd tests && ./test.sh