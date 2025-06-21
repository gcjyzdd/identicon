# identicon

Generate identity icon

This repository contains a small Rust crate for creating GitLab-style SVG
identicons. The crate lives in `gitlab_identicon/` and provides a single
function `generate_svg` that returns an SVG image for a given input string.

```bash
cargo run -p gitlab_identicon -- example > example.svg
```

The generated SVG represents a 5x5 grid with colors and patterns derived from
the SHA‑256 hash of the input.
