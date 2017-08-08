[![Build Status](https://travis-ci.org/jean553/rust-a-star.svg?branch=master)](https://travis-ci.org/jean553/rust-a-star)

# Rust A*

Attempt to implement the A-star algorithm in Rust with graphical resolution.

## Compilation and development

Start the docker container.

```bash
vagrant up
```

Connect to the container.

```bash
vagrant ssh
```

Compile the program.

```bash
cargo build --release
```

## Run

On your host:

```bash
./rust-a-star/target/release/rust-a-star
```

## Generate documentation

```bash
cargo rustdoc -- --no-defaults
```

## Credits

* https://www.iconfinder.com/icons/2252378/pin_pin_it_pinterest_tag_icon - Pin by Vectto (https://creativecommons.org/licenses/by/3.0/)
