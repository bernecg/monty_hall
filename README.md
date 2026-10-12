# Monty Hall Simulation

This project simulates the Monty Hall problem in Rust.

The player chooses one door, Monty eliminates losing doors while leaving one
alternative available, and the simulation compares two strategies:

- staying with the initial choice;
- switching to the remaining alternative.

The simulation demonstrates that switching wins more often. With three doors,
switching wins approximately 66.67% of the time while staying wins
approximately 33.33% of the time. With more doors, the advantage of switching
becomes even larger.

## Running the simulation

Install Rust, then run:

```bash
cargo run
```

Enter the number of doors when prompted. The minimum number of doors is `3`.
Enter `0` to quit.

The simulation runs `100,000` rounds for each selected door count.

## Testing

Run all tests with:

```bash
cargo test
```

Check that the code is formatted with:

```bash
cargo fmt --all --check
```

Build the generated Rust documentation with:

```bash
cargo doc --open
```

## GitHub Actions

The repository includes a GitHub Actions workflow that runs the formatting
check and test suite on pushes and pull requests.

## License

This project is a learning exercise and does not currently specify a license.
