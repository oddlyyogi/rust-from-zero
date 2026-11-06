# 26 - Testing and Documentation

How do you know your code still works after you change it? A known test weight on a depot scale explains testing. We write unit tests with assert_eq, watch one catch a real pricing bug, use assert, should_panic and tests that return Result, run only some tests, add integration tests in a tests folder, write documentation examples that run as tests, and generate a documentation site with cargo doc.

## Run it

```bash
cargo run
```

This folder holds the program as it stands at the end of the chapter.

## Commands used in the video

```bash
cargo test
cargo test heavy
cargo test -- --test-threads=1
cargo test -- --ignored
cargo doc --no-deps
```

## Sources

- How to write tests, assert_eq, should_panic, tests returning Result: https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html
- Controlling how tests run, filtering, parallel threads, hidden output: https://doc.rust-lang.org/stable/book/ch11-02-running-tests.html
- Test organization, unit and integration tests, cfg(test): https://doc.rust-lang.org/stable/book/ch11-03-test-organization.html
- Documentation comments and doc tests: https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html
- cargo doc: https://doc.rust-lang.org/stable/cargo/commands/cargo-doc.html
- docs.rs builds documentation for published crates: https://docs.rs/about
