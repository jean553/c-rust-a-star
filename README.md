# C Rust A*

A graphical C program showing `A*` paths resolution using a Rust library.

## Requirements

The following tools are required:
 * cargo (Rust nightly),
 * gcc

## Compile

The following command compiles both
of the C program and the Rust library.

```sh
make
```

## Run

```sh
./c_rust_a_star 10 10 5 6
```

The options are (in order):
 * map width (in nodes),
 * map height (in nodes),
 * departure node index,
 * arrival node index

## Remove previous compilation data

```sh
make clean
```
