# lasConverter

A simple tool to convert LAS files (.las) to a custom binary format.

## Dependencies

This project uses the following Rust crate:
- [las](https://crates.io/crates/las)

## Building

To build the project, use the following command:

```bash
cargo build --release
```

The executable will be located in `target/release/lasConverter`.

## Usage

Currently, the input and output file paths are hardcoded in `src/main.rs`.

To convert a file, modify the `main` function in `src/main.rs` to specify your input and output files:

```rust
fn main() {
    let converter = LasConverter::new();
    converter.convert_to_bin("./path/to/your/file.las", "./path/to/your/output.bin", false);
}
```

Then, run the executable:

```bash
./target/release/lasConverter
```

### Future Improvements

For more flexible usage, you could modify the `main` function to accept command-line arguments for the input and output file paths.

## Output Format

The output binary file consists of a series of 3D points. Each point is represented by a `Point3D` struct with the following format:

- `x`: 32-bit float
- `y`: 32-bit float
- `z`: 32-bit float

The points are written to the file sequentially. ( TODO make threads xd )
