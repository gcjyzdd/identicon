use sha2::{Digest, Sha256};

/// Generate a GitLab-style identicon SVG for the given input string.
/// `size` defines both width and height of the resulting square image in pixels.
///
/// The output is a simple 5x5 grid with cells mirrored horizontally.
/// The fill color and pattern are derived from the SHA-256 hash of `input`.
pub fn generate_svg(input: &str, size: u32) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();
    // First three bytes define the color
    let color = format!("#{:02x}{:02x}{:02x}", hash[0], hash[1], hash[2]);

    // Determine which of the 15 unique cells are filled
    let mut cells = [false; 15];
    for i in 0..15 {
        cells[i] = hash[3 + i] % 2 == 0;
    }

    let cell_size = size / 5;
    let mut rects = String::new();
    let mut idx = 0;
    for row in 0..5 {
        for col in 0..3 {
            if cells[idx] {
                let x = col * cell_size;
                let y = row * cell_size;
                rects.push_str(&format!(
                    "<rect x=\"{x}\" y=\"{y}\" width=\"{cs}\" height=\"{cs}\" fill=\"{color}\" />",
                    x = x,
                    y = y,
                    cs = cell_size
                ));
                let mirror_x = (4 - col) * cell_size;
                if mirror_x != x {
                    rects.push_str(&format!(
                        "<rect x=\"{x}\" y=\"{y}\" width=\"{cs}\" height=\"{cs}\" fill=\"{color}\" />",
                        x = mirror_x,
                        y = y,
                        cs = cell_size
                    ));
                }
            }
            idx += 1;
        }
    }

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{size}\" height=\"{size}\" viewBox=\"0 0 {size} {size}\">{rects}</svg>",
        size = size,
        rects = rects
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consistent_output() {
        let svg1 = generate_svg("alice", 100);
        let svg2 = generate_svg("alice", 100);
        assert_eq!(svg1, svg2);
    }
}
