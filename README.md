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
