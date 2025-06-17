SHELL := /bin/bash

compile_program:
	cd program && \
	cargo +valida build --release && \
	cp target/valida-unknown-baremetal-gnu/release/program ../test_data/program.bin

program_to_base64: compile_program
	cd program && \
	base64 ./program.bin > program.base64

run_program: compile_program
	cat test_data/stdin | valida run ./test_data/program.bin test_data/stdout

prove_program: compile_program
	valida prove ./program/program.bin ./test_data/proof.bin ./test_data/stdin

verify_program:
	cargo run --release --manifest-path ./verifier/Cargo.toml

compile_front_end: program_to_base64
	cp program/program.base64 ./app/program.base64
	cd app && \
	npm install && npm run start
