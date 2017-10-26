CC=gcc
SOURCEDIR=src
LIBS=-lSDL2

all: c_rust_a_star a_star_rs

c_rust_a_star:
	$(CC) $(SOURCEDIR)/main.c -o c_rust_a_star $(LIBS)

a_star_rs:
	cargo build --manifest-path libs/a-star-rs/a-star-rs/Cargo.toml
