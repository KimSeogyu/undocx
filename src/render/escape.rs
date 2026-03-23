/// Escapes a string for safe use inside an HTML attribute value.
///
/// Replaces `&`, `<`, `>`, `"`, and `'` with their HTML entity equivalents.
pub fn escape_html_attr(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

/// Escapes a string for use inside Markdown link text (`[text](url)`).
///
/// Backslash-escapes `\`, `[`, and `]`.
pub fn escape_markdown_link_text(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' | '[' | ']' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}

/// Escapes a string for use inside a Markdown link destination (`[text](url)`).
///
/// Backslash-escapes `\`, `(`, `)`, and spaces.
pub fn escape_markdown_link_destination(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' | '(' | ')' | ' ' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html_attr() {
        assert_eq!(
            escape_html_attr("a\"b<c>d&e'"),
            "a&quot;b&lt;c&gt;d&amp;e&#39;"
        );
    }

    #[test]
    fn test_escape_markdown_link_text() {
        assert_eq!(escape_markdown_link_text("A[B]"), "A\\[B\\]");
    }

    #[test]
    fn test_escape_markdown_link_destination() {
        assert_eq!(
            escape_markdown_link_destination("https://x.y/a b(c)"),
            "https://x.y/a\\ b\\(c\\)"
        );
    }
}
