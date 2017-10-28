# C Rust A*

A graphical C program showing `A*` paths resolution using a Rust library.

## Requirements

The following tools are required:
 * cargo (Rust nightly),
 * gcc

## Install sub-module (Rust A-Star library)

```bash
git submodule init
git submodule update
```

## Compile

The following command compiles both
of the C program and the Rust library.

```bash
make
```

## Run

```bash
./c_rust_a_star 10 10 5 6
```

The options are (in order):
 * map width (in nodes),
 * map height (in nodes),
 * departure node index,
 * arrival node index
