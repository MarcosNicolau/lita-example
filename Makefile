SHELL := /bin/bash

compile_program:
	cd program && \
	cargo +valida build --release && \
	cp target/release/valida-unknown-baremetal-gnu/release/program ./program.bin

program_to_base64: compile_program
	cd program && \
	base64 ./program.bin > program.base64

prove_program: compile_program
	valida prove ./program/program.bin ./test_data/proof.bin ./test_data/stdin

verify_program:
	cd verifier && cargo run --release

compile_front_end: program_to_base64
	cp program/program.base64 ./app/program.base64
	cd app && \
	npm install && npm run start
