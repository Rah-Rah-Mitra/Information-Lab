//! Math-density heuristic for ingest chunks.
//!
//! `pdf_oxide` gives us per-page markdown — clean enough for the reasoner
//! but lossy on equations (display math often lands as stray glyph runs or
//! `[math]` placeholders). We need a cheap, local signal to decide which
//! chunks deserve a follow-up pass by the light-tier FormulaExtractor
//! agent, without paying an LLM call on every ingested chunk.
//!
//! The score is a density (matches per character, clamped to `[0, 1]`) so
//! a single threshold works across chunk sizes. Tuned empirically against
//! the CV / Robotics corpus in `public/`.

/// Unicode glyphs that strongly suggest display math.
const MATH_GLYPHS: &[char] = &[
    '∑', '∫', '∂', '∇', '√', '∞', '≤', '≥', '≠', '≈', '∈', '∉', '∪', '∩',
    '⋅', '⊕', '⊗', '→', '↔', '⇒', '⇔', 'α', 'β', 'γ', 'δ', 'ε', 'θ', 'λ',
    'μ', 'π', 'σ', 'τ', 'φ', 'χ', 'ψ', 'ω', 'Γ', 'Δ', 'Θ', 'Λ', 'Π', 'Σ',
    'Φ', 'Ψ', 'Ω',
];

/// LaTeX command fragments that survive pdf_oxide's extraction.
const LATEX_HINTS: &[&str] = &[
    "\\frac", "\\sum", "\\int", "\\prod", "\\partial", "\\nabla", "\\sqrt",
    "\\infty", "\\alpha", "\\beta", "\\gamma", "\\delta", "\\theta",
    "\\lambda", "\\mu", "\\pi", "\\sigma", "\\phi", "\\mathbf", "\\mathrm",
    "\\begin{equation}", "\\begin{align}",
];

/// Score a chunk in `[0.0, 1.0]`. Higher means more math-dense.
///
/// Combines three signals:
///
/// * density of Unicode math glyphs per character,
/// * presence of explicit LaTeX delimiters (`$…$`, `$$…$$`, `\(…\)`),
/// * presence of `\cmd` fragments from [`LATEX_HINTS`].
///
/// Short chunks are floored at length 200 so a stray glyph on a 40-char
/// fragment doesn't score 1.0.
pub fn math_density_score(text: &str) -> f32 {
    if text.trim().is_empty() {
        return 0.0;
    }
    let len = text.chars().count().max(200) as f32;

    let glyph_hits = text.chars().filter(|c| MATH_GLYPHS.contains(c)).count() as f32;
    let glyph_density = (glyph_hits / len).min(1.0);

    let delim_hits = count_delims(text) as f32;
    let delim_signal = (delim_hits / 4.0).min(1.0);

    let hint_hits = LATEX_HINTS.iter().filter(|h| text.contains(**h)).count() as f32;
    let hint_signal = (hint_hits / 3.0).min(1.0);

    // Weighted mix: glyphs dominate (they're the pdf_oxide-visible signal)
    // but even one `\frac` or `$$` block is a strong positive indicator.
    let score = glyph_density * 4.0 + delim_signal * 0.4 + hint_signal * 0.4;
    score.clamp(0.0, 1.0)
}

fn count_delims(text: &str) -> usize {
    // Count `$$` pairs (display) and `$…$`/`\(…\)` inline opens — cheap scan.
    let mut n = text.matches("$$").count() / 2;
    n += text.matches("\\(").count();
    n += text.matches("\\[").count();
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_is_low() {
        let s = "The quick brown fox jumps over the lazy dog. ".repeat(8);
        assert!(math_density_score(&s) < 0.1, "got {}", math_density_score(&s));
    }

    #[test]
    fn greek_heavy_is_high() {
        let s = "Let α, β, γ ∈ ℝ. Then ∑ α_i · β_i ≥ 0 and ∂f/∂x → ∞ as x → ∞. ".repeat(4);
        assert!(math_density_score(&s) > 0.2, "got {}", math_density_score(&s));
    }

    #[test]
    fn latex_block_triggers() {
        let s = "Normal text. $$\\sum_{i=1}^{n} x_i = \\mu$$ more text. ".repeat(3);
        assert!(math_density_score(&s) > 0.1, "got {}", math_density_score(&s));
    }
}
