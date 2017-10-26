CC=gcc
SOURCEDIR=src
LIBS=-lSDL2 libs/a-star-rs/a-star-rs/target/debug/libastar.a

all: a_star_rs c_rust_a_star

c_rust_a_star:
	$(CC) $(SOURCEDIR)/main.c -o c_rust_a_star $(LIBS)

a_star_rs:
	cargo build --manifest-path libs/a-star-rs/a-star-rs/Cargo.toml
