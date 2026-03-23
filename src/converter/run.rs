//! Converts a DOCX `<w:run>` into Markdown-level inline markup.
//!
//! A "run" is OOXML's unit of consistent character formatting — a contiguous
//! span where bold/italic/font/etc. don't change. This module decides how
//! to map those properties to Markdown: monospace fonts become backtick code,
//! bold becomes `<strong>`, and so on.

use super::ConversionContext;
use crate::localization::is_monospace_font;
use crate::Result;
use rs_docx::document::{BreakType, Run, RunContent};

pub struct RunConverter;

impl RunConverter {
    pub fn convert<'a>(
        run: &Run<'a>,
        context: &mut ConversionContext<'a>,
        para_style_id: Option<&str>,
    ) -> Result<String> {
        let mut text = String::new();

        // Extract text from run content
        for content in &run.content {
            match content {
                RunContent::Text(t) => {
                    text.push_str(&t.text);
                }
                RunContent::Break(br) => match br.ty {
                    Some(BreakType::Page) => text.push_str("\n\n---\n\n"),
                    Some(BreakType::Column) => text.push_str("\n\n"),
                    _ => text.push('\n'),
                },
                RunContent::Tab(_) => {
                    text.push('\t');
                }
                RunContent::CarriageReturn(_) => {
                    text.push('\n');
                }
                RunContent::NoBreakHyphen(_) => {
                    text.push('\u{2011}');
                }
                RunContent::SoftHyphen(_) => {
                    text.push('\u{00AD}');
                }
                RunContent::Drawing(drawing) => {
                    if let Some(img_md) = context.extract_image_from_drawing(drawing)? {
                        text.push_str(&img_md);
                    }
                }
                RunContent::Pict(pict) => {
                    if let Some(img_md) = context.extract_image_from_pict(pict)? {
                        text.push_str(&img_md);
                    }
                }
                RunContent::Sym(sym) => {
                    if let Some(char_code) = &sym.char {
                        if let Ok(code) = u32::from_str_radix(char_code, 16) {
                            if let Some(c) = char::from_u32(code) {
                                text.push(c);
                            }
                        }
                    }
                }
                RunContent::FootnoteReference(fnref) => {
                    if let Some(id_str) = &fnref.id {
                        if let Ok(id_num) = id_str.parse::<isize>() {
                            let marker = context.register_footnote_reference(id_num);
                            text.push_str(&marker);
                        }
                    }
                }
                RunContent::EndnoteReference(enref) => {
                    if let Some(id_str) = &enref.id {
                        if let Ok(id_num) = id_str.parse::<isize>() {
                            let marker = context.register_endnote_reference(id_num);
                            text.push_str(&marker);
                        }
                    }
                }
                RunContent::CommentReference(cref) => {
                    if let Some(id) = &cref.id {
                        let marker = context.register_comment_reference(id.as_ref());
                        text.push_str(&marker);
                    }
                }
                RunContent::PTab(_) => {
                    text.push('\t');
                }
                RunContent::LastRenderedPageBreak(_) => {
                    text.push_str("\n\n---\n\n");
                }
                RunContent::PgNum(_) => {
                    text.push_str("{PAGE}");
                }
                RunContent::AnnotationRef(_)
                | RunContent::FootnoteRef(_)
                | RunContent::EndnoteRef(_)
                | RunContent::Separator(_)
                | RunContent::ContinuationSeparator(_) => {}
                _ => {}
            }
        }

        if text.is_empty() {
            return Ok(text);
        }

        let mut run_style_id = None;
        if let Some(props) = &run.property {
            if let Some(style) = &props.style_id {
                run_style_id = Some(style.value.as_ref());
            }
        }

        let effective_props =
            context.resolve_run_property(run.property.as_ref(), run_style_id, para_style_id);

        text = Self::apply_formatting(&text, &effective_props, context);

        Ok(text)
    }

    /// Formatting precedence (highest priority first):
    /// monospace → inline code, then underline → strike → bold/italic → super/subscript.
    /// Monospace short-circuits: if a run uses a code font, no other formatting is applied.
    fn apply_formatting(
        text: &str,
        props: &rs_docx::formatting::CharacterProperty<'_>,
        context: &ConversionContext<'_>,
    ) -> String {
        let mut result = text.to_string();

        let is_code = props.fonts.as_ref().map(is_monospace_font).unwrap_or(false);
        if is_code {
            return format!("`{}`", result);
        }

        let is_bold = props
            .bold
            .as_ref()
            .map(|b| b.value.unwrap_or(true))
            .unwrap_or(false);
        let is_italic = props
            .italics
            .as_ref()
            .map(|i| i.value.unwrap_or(true))
            .unwrap_or(false);

        let has_underline = props.underline.is_some();
        let has_strike = props
            .strike
            .as_ref()
            .map(|s| s.value.unwrap_or(true))
            .unwrap_or(false);

        if has_underline && context.html_underline_enabled() {
            result = format!("<u>{}</u>", result);
        }

        if has_strike {
            if context.html_strikethrough_enabled() {
                result = format!("<s>{}</s>", result);
            } else {
                result = format!("~~{}~~", result);
            }
        }

        if is_bold && is_italic {
            result = format!("<strong><em>{}</em></strong>", result);
        } else if is_bold {
            result = format!("<strong>{}</strong>", result);
        } else if is_italic {
            result = format!("<em>{}</em>", result);
        }

        let is_superscript = props
            .vertical_align
            .as_ref()
            .and_then(|v| v.value.as_ref())
            .map(|v| matches!(v, rs_docx::formatting::VertAlignType::Superscript))
            .unwrap_or(false);
        let is_subscript = props
            .vertical_align
            .as_ref()
            .and_then(|v| v.value.as_ref())
            .map(|v| matches!(v, rs_docx::formatting::VertAlignType::Subscript))
            .unwrap_or(false);

        if is_superscript {
            result = format!("<sup>{}</sup>", result);
        }
        if is_subscript {
            result = format!("<sub>{}</sub>", result);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hard_xml::XmlRead;
    use rs_docx::document::{Run, RunContent, Text};
    use rs_docx::formatting::{Bold, CharacterProperty, Italics};

    #[test]
    fn test_plain_text_run() {
        make_test_context!(ctx);
        let mut run = Run::default();
        run.content.push(RunContent::Text(Text {
            text: "hello".into(),
            ..Default::default()
        }));
        let result = RunConverter::convert(&run, &mut ctx, None).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_bold_run() {
        make_test_context!(ctx);
        let mut run = Run {
            property: Some(CharacterProperty {
                bold: Some(Bold { value: Some(true) }),
                ..Default::default()
            }),
            ..Default::default()
        };
        run.content.push(RunContent::Text(Text {
            text: "hello".into(),
            ..Default::default()
        }));
        let result = RunConverter::convert(&run, &mut ctx, None).unwrap();
        assert_eq!(result, "<strong>hello</strong>");
    }

    #[test]
    fn test_italic_run() {
        make_test_context!(ctx);
        let mut run = Run {
            property: Some(CharacterProperty {
                italics: Some(Italics { value: Some(true) }),
                ..Default::default()
            }),
            ..Default::default()
        };
        run.content.push(RunContent::Text(Text {
            text: "hello".into(),
            ..Default::default()
        }));
        let result = RunConverter::convert(&run, &mut ctx, None).unwrap();
        assert_eq!(result, "<em>hello</em>");
    }

    #[test]
    fn test_superscript_run() {
        use rs_docx::formatting::{CharacterProperty, VertAlign, VertAlignType};

        make_test_context!(ctx);
        let mut run = Run {
            property: Some(CharacterProperty {
                vertical_align: Some(VertAlign { value: Some(VertAlignType::Superscript) }),
                ..Default::default()
            }),
            ..Default::default()
        };
        run.content.push(RunContent::Text(Text { text: "n".into(), ..Default::default() }));
        let result = RunConverter::convert(&run, &mut ctx, None).unwrap();
        assert_eq!(result, "<sup>n</sup>");
    }

    #[test]
    fn test_subscript_run() {
        use rs_docx::formatting::{CharacterProperty, VertAlign, VertAlignType};

        make_test_context!(ctx);
        let mut run = Run {
            property: Some(CharacterProperty {
                vertical_align: Some(VertAlign { value: Some(VertAlignType::Subscript) }),
                ..Default::default()
            }),
            ..Default::default()
        };
        run.content.push(RunContent::Text(Text { text: "2".into(), ..Default::default() }));
        let result = RunConverter::convert(&run, &mut ctx, None).unwrap();
        assert_eq!(result, "<sub>2</sub>");
    }

    #[test]
    fn test_monospace_font_run_produces_code() {
        use rs_docx::formatting::{CharacterProperty, Fonts};

        make_test_context!(ctx);
        let mut run = Run {
            property: Some(CharacterProperty {
                fonts: Some(Fonts {
                    ascii: Some("Courier New".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        run.content.push(RunContent::Text(Text { text: "x = 1".into(), ..Default::default() }));
        let result = RunConverter::convert(&run, &mut ctx, None).unwrap();
        assert_eq!(result, "`x = 1`");
    }

    #[test]
    fn test_sym_character() {
        make_test_context!(ctx);
        let run = Run::from_str(
            r#"<w:r xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:sym w:char="2022"/></w:r>"#,
        )
        .unwrap();
        let result = RunConverter::convert(&run, &mut ctx, None).unwrap();
        assert!(result.contains('\u{2022}'));
    }
}
