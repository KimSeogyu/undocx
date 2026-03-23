//! Heading style parsing utilities.

/// Parses a DOCX style name to determine if it's a blockquote style.
pub fn is_blockquote_style(style: &str) -> bool {
    let style_lower = style.to_lowercase();
    matches!(
        style_lower.as_str(),
        "quote" | "intensequote" | "intense quote" | "blocktext" | "block text"
    )
}

/// Returns true if the style is a code/preformatted style.
pub fn is_code_style(style: &str) -> bool {
    let s = style.to_lowercase();
    matches!(
        s.as_str(),
        "code"
            | "sourcecode"
            | "source code"
            | "htmlpreformatted"
            | "html preformatted"
            | "plaintext"
            | "plain text"
            | "macro"
            | "macrotext"
            | "macro text"
    )
}

/// Returns true if the font name is a monospace/code font.
pub fn is_monospace_font_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("courier")
        || lower.contains("consolas")
        || lower.contains("mono")
        || lower.contains("source code")
        || lower.contains("fira code")
        || lower.contains("menlo")
        || lower.contains("dejavu sans mono")
        || lower.contains("liberation mono")
        || lower.contains("andale mono")
        || lower.contains("lucida console")
}

/// Parses a DOCX style name to determine the heading level.
///
/// Recognizes standard heading styles like "Heading1", "Heading2", etc.,
/// as well as "Title" (level 1) and "Subtitle" (level 2).
///
/// Returns `None` if the style is not recognized as a heading.
pub fn parse_heading_style(style: &str) -> Option<usize> {
    let style_lower = style.to_lowercase();

    // Standard headings: "Heading1", "Heading 1", "heading1", etc.
    if let Some(rest) = style_lower.strip_prefix("heading") {
        return rest.trim().parse().ok();
    }

    // Common title styles
    match style_lower.as_str() {
        "title" => Some(1),
        "subtitle" => Some(2),
        _ => None,
    }
}
