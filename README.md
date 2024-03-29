# Ergosim :: game :: simulator

## Dependencies

### Production 

#### [Rand](https://crates.io/crates/rand)

Enables random number generation and associated functionality, e.g. sampling a normal distribution over a 
range of numbers.

#### [Rust Decimal](https://crates.io/crates/rust_decimal)

Enables the use of fixed precision decimal numbers in Rust, which is useful to represent results of financial
calculations.

### Development

#### [QuickCheck](https://crates.io/crates/quickcheck)

Property based testing for Rust.

#### [QuickCheck Macros](https://crates.io/crates/quickcheck_macros)

Some utility macros for the quickcheck crate.

## Managing the project

### Building

```shell
wasm-pack build
```

### Testing

#### Property based testing

Rather than testing specific inputs and expecting particular outputs, we
valid that a randomly created input is well behaved in the system. We
define a number of properties that always should hold. Then, instead of
testing manually constructed example cases, we test that these
properties hold for every randomly created input.

_The tricky part_ is to imagine what properties should hold, which 
requires a shift in thinking.

##### Libraries

See [this article](https://www.greyblake.com/blog/property-based-testing-in-rust-with-arbitrary/)
discussing property based testing in Rust.

These two crates can be used to proceed:

- https://crates.io/crates/arbitrary
- https://crates.io/crates/arbtest

Another [article](https://www.lpalmieri.com/posts/an-introduction-to-property-based-testing-in-rust/#how-to-generate-random-test-data-with-faker)
discusses property based testing in Rust. This post suggests one of the
following crates:

- [QuickCheck](https://crates.io/crates/quickcheck)
- [PropTest](https://docs.rs/proptest/latest/proptest/)