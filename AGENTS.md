# Guidelines for Contributors

This project provides a small Rust crate for generating GitLab-style identicons.
Follow these guidelines when modifying or extending the code:

- **Security first**: prefer safe Rust. Avoid `unsafe` blocks unless absolutely
  necessary and justify their use in code comments.
- **Minimal dependencies**: keep the dependency list short and well maintained.
  Avoid including crates that are not actively maintained or widely used.
- **Error handling**: use `Result` and `Option` types rather than panicking. All
  public functions should return meaningful errors.
- **Formatting and linting**:
  - Run `cargo fmt --all` to ensure consistent code style.
  - Run `cargo clippy --all-targets --all-features` and fix warnings.
- **Testing**: run `cargo test --manifest-path gitlab_identicon/Cargo.toml` and
  ensure all tests pass before committing changes.
- **Documentation**: document public types and functions with `///` comments and
  keep the README up to date with usage examples.

