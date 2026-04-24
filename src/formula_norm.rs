//! Canonical normalization for formula dedupe keys (`latex_norm`).

/// Build a stable dedupe key from a LaTeX string:
/// - trim outer whitespace
/// - remove common wrapping math delimiters (`$$...$$`, `\(...\)`, `\[...\]`, `$...$`)
/// - collapse all whitespace runs to a single space
pub fn normalize_latex_for_dedupe(latex: &str) -> String {
    let mut out = latex.trim().to_string();
    loop {
        let trimmed = out.trim();
        let next = if trimmed.starts_with("$$")
            && trimmed.ends_with("$$")
            && trimmed.len() >= 4
        {
            trimmed[2..trimmed.len() - 2].trim()
        } else if trimmed.starts_with("\\(")
            && trimmed.ends_with("\\)")
            && trimmed.len() >= 4
        {
            trimmed[2..trimmed.len() - 2].trim()
        } else if trimmed.starts_with("\\[")
            && trimmed.ends_with("\\]")
            && trimmed.len() >= 4
        {
            trimmed[2..trimmed.len() - 2].trim()
        } else if trimmed.starts_with('$')
            && trimmed.ends_with('$')
            && trimmed.len() >= 2
        {
            trimmed[1..trimmed.len() - 1].trim()
        } else {
            trimmed
        };
        if next == out {
            break;
        }
        out = next.to_string();
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::normalize_latex_for_dedupe;

    #[test]
    fn near_duplicates_with_same_symbols_get_distinct_norms() {
        let a = normalize_latex_for_dedupe(r"a + b = c");
        let b = normalize_latex_for_dedupe(r"a - b = c");
        assert_ne!(a, b);
    }

    #[test]
    fn equivalent_spacing_variants_collapse_to_same_norm() {
        let a = normalize_latex_for_dedupe(r" \[  E = m^2 c   \] ");
        let b = normalize_latex_for_dedupe("\nE\t=\tm^2   c\n");
        assert_eq!(a, b);
    }

    #[test]
    fn nested_delimiters_are_unwrapped() {
        let got = normalize_latex_for_dedupe(r"$$ \( x + y \) $$");
        assert_eq!(got, "x + y");
    }
}
