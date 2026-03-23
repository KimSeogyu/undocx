use super::AstExtractor;
use crate::converter::{ConversionContext, ParagraphConverter, RunConverter, TableConverter};
use crate::core::ast::{BlockNode, DocumentAst};
use crate::render::escape_html_attr;
use crate::Result;
use rs_docx::document::{BodyContent, TableCell, TableCellContent};

#[derive(Debug, Default, Clone, Copy)]
pub struct DocxExtractor;

impl AstExtractor for DocxExtractor {
    fn extract<'a>(
        &self,
        body: &[BodyContent<'a>],
        context: &mut ConversionContext<'a>,
    ) -> Result<DocumentAst> {
        let mut doc = DocumentAst::default();
        for content in body {
            self.extract_content(content, context, &mut doc)?;
        }
        doc.blocks = Self::merge_code_blocks(doc.blocks);
        Ok(doc)
    }
}

impl DocxExtractor {
    fn extract_table_cell<'a>(
        &self,
        cell: &TableCell<'a>,
        context: &mut ConversionContext<'a>,
        output: &mut DocumentAst,
    ) -> Result<()> {
        for item in &cell.content {
            match item {
                TableCellContent::Paragraph(para) => {
                    let converted = ParagraphConverter::convert(para, context)?;
                    if !converted.is_empty() {
                        output.blocks.push(BlockNode::Paragraph(converted));
                    }
                }
                TableCellContent::Table(table) => {
                    let converted = TableConverter::convert(table, context)?;
                    output.blocks.push(BlockNode::TableHtml(converted));
                }
            }
        }
        Ok(())
    }

    fn is_code_paragraph(block: &BlockNode) -> bool {
        match block {
            BlockNode::Paragraph(s) => s.starts_with("\u{200B}CODE:"),
            _ => false,
        }
    }

    fn strip_code_marker(s: &str) -> &str {
        s.strip_prefix("\u{200B}CODE:").unwrap_or(s)
    }

    fn merge_code_blocks(blocks: Vec<BlockNode>) -> Vec<BlockNode> {
        let mut result: Vec<BlockNode> = Vec::new();
        let mut code_lines: Vec<String> = Vec::new();

        for block in blocks {
            if Self::is_code_paragraph(&block) {
                let text = match &block {
                    BlockNode::Paragraph(s) => Self::strip_code_marker(s).to_string(),
                    _ => unreachable!(),
                };
                code_lines.push(text);
            } else {
                if !code_lines.is_empty() {
                    result.push(BlockNode::RawHtml(format!(
                        "```\n{}\n```",
                        code_lines.join("\n")
                    )));
                    code_lines.clear();
                }
                result.push(block);
            }
        }

        if !code_lines.is_empty() {
            result.push(BlockNode::RawHtml(format!(
                "```\n{}\n```",
                code_lines.join("\n")
            )));
        }

        result
    }

    fn is_all_monospace_paragraph(para: &rs_docx::document::Paragraph<'_>) -> bool {
        let mut has_text = false;
        for content in &para.content {
            if let rs_docx::document::ParagraphContent::Run(run) = content {
                let has_text_content = run.content.iter().any(|c| {
                    matches!(c, rs_docx::document::RunContent::Text(_))
                });
                if !has_text_content {
                    continue;
                }
                has_text = true;

                let is_mono = run
                    .property
                    .as_ref()
                    .and_then(|p| p.fonts.as_ref())
                    .map(|f| {
                        f.ascii
                            .as_ref()
                            .map(|n| crate::localization::is_monospace_font_name(n))
                            .unwrap_or(false)
                            || f.h_ansi
                                .as_ref()
                                .map(|n| crate::localization::is_monospace_font_name(n))
                                .unwrap_or(false)
                    })
                    .unwrap_or(false);

                if !is_mono {
                    return false;
                }
            }
        }
        has_text
    }

    fn extract_content<'a>(
        &self,
        content: &BodyContent<'a>,
        context: &mut ConversionContext<'a>,
        output: &mut DocumentAst,
    ) -> Result<()> {
        match content {
            BodyContent::Paragraph(para) => {
                let is_code_style = para
                    .property
                    .as_ref()
                    .and_then(|p| p.style_id.as_ref())
                    .map(|s| crate::localization::is_code_style(&s.value))
                    .unwrap_or(false);

                let is_all_monospace = !is_code_style && Self::is_all_monospace_paragraph(para);

                if is_code_style || is_all_monospace {
                    let raw_text = para.text();
                    output
                        .blocks
                        .push(BlockNode::Paragraph(format!("\u{200B}CODE:{}", raw_text)));
                } else {
                    let converted = ParagraphConverter::convert(para, context)?;
                    if !converted.is_empty() {
                        output.blocks.push(BlockNode::Paragraph(converted));
                    }
                }
            }
            BodyContent::Table(table) => {
                let converted = TableConverter::convert(table, context)?;
                output.blocks.push(BlockNode::TableHtml(converted));
            }
            BodyContent::Run(run) => {
                let converted = RunConverter::convert(run, context, None)?;
                if !converted.is_empty() {
                    output.blocks.push(BlockNode::Paragraph(converted));
                }
            }
            BodyContent::TableCell(cell) => {
                self.extract_table_cell(cell, context, output)?;
            }
            BodyContent::Sdt(sdt) => {
                if let Some(sdt_content) = &sdt.content {
                    for child in &sdt_content.content {
                        self.extract_content(child, context, output)?;
                    }
                }
            }
            BodyContent::BookmarkStart(bookmark) => {
                if let Some(name) = &bookmark.name {
                    output.blocks.push(BlockNode::RawHtml(format!(
                        "<a id=\"{}\"></a>",
                        escape_html_attr(name)
                    )));
                }
            }
            BodyContent::BookmarkEnd(_) => {}
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::converter::{ConversionContext, ImageExtractor, NumberingResolver, StyleResolver};
    use rs_docx::document::{BodyContent, Paragraph, ParagraphContent, Run, RunContent, Text};
    use rs_docx::formatting::{ParagraphProperty, ParagraphStyleId};
    use std::collections::HashMap;

    #[test]
    fn test_consecutive_code_paragraphs_grouped_into_code_block() {
        let mut body: Vec<BodyContent> = Vec::new();

        for line in ["line 1", "line 2", "line 3"] {
            let mut para = Paragraph {
                property: Some(ParagraphProperty {
                    style_id: Some(ParagraphStyleId {
                        value: "Code".into(),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            };
            let mut run = Run::default();
            run.content.push(RunContent::Text(Text {
                text: line.into(),
                ..Default::default()
            }));
            para.content.push(ParagraphContent::Run(run));
            body.push(BodyContent::Paragraph(para));
        }

        let docx = rs_docx::Docx::default();
        let rels = HashMap::new();
        let mut numbering_resolver = NumberingResolver::new(&docx);
        let mut image_extractor = ImageExtractor::new_skip();
        let options = crate::ConvertOptions::default();
        let style_resolver = StyleResolver::new(&docx.styles);
        let mut context = ConversionContext::new(
            &rels,
            &mut numbering_resolver,
            &mut image_extractor,
            &options,
            None,
            None,
            None,
            &style_resolver,
        );

        let extractor = DocxExtractor;
        let doc = extractor.extract(&body, &mut context).unwrap();

        let combined = doc
            .blocks
            .iter()
            .map(|b| match b {
                crate::core::ast::BlockNode::Paragraph(s)
                | crate::core::ast::BlockNode::TableHtml(s)
                | crate::core::ast::BlockNode::RawHtml(s) => s.as_str(),
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        assert!(
            combined.contains("```"),
            "Expected fenced code block, got: {}",
            combined
        );
        assert!(combined.contains("line 1"), "Missing line 1");
        assert!(combined.contains("line 2"), "Missing line 2");
        assert!(combined.contains("line 3"), "Missing line 3");
    }

    #[test]
    fn test_single_code_paragraph_still_produces_code_block() {
        let mut body: Vec<BodyContent> = Vec::new();

        let mut para = Paragraph {
            property: Some(ParagraphProperty {
                style_id: Some(ParagraphStyleId {
                    value: "SourceCode".into(),
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut run = Run::default();
        run.content.push(RunContent::Text(Text {
            text: "print('hello')".into(),
            ..Default::default()
        }));
        para.content.push(ParagraphContent::Run(run));
        body.push(BodyContent::Paragraph(para));

        let docx = rs_docx::Docx::default();
        let rels = HashMap::new();
        let mut numbering_resolver = NumberingResolver::new(&docx);
        let mut image_extractor = ImageExtractor::new_skip();
        let options = crate::ConvertOptions::default();
        let style_resolver = StyleResolver::new(&docx.styles);
        let mut context = ConversionContext::new(
            &rels,
            &mut numbering_resolver,
            &mut image_extractor,
            &options,
            None,
            None,
            None,
            &style_resolver,
        );

        let extractor = DocxExtractor;
        let doc = extractor.extract(&body, &mut context).unwrap();

        let combined = doc
            .blocks
            .iter()
            .map(|b| match b {
                crate::core::ast::BlockNode::Paragraph(s)
                | crate::core::ast::BlockNode::TableHtml(s)
                | crate::core::ast::BlockNode::RawHtml(s) => s.as_str(),
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        assert!(
            combined.contains("```"),
            "Single code paragraph should produce code block, got: {}",
            combined
        );
        assert!(
            combined.contains("print('hello')"),
            "Missing code content"
        );
    }

    #[test]
    fn test_code_paragraphs_separated_by_normal_are_separate_blocks() {
        let mut body: Vec<BodyContent> = Vec::new();

        // Code paragraph
        let mut para1 = Paragraph {
            property: Some(ParagraphProperty {
                style_id: Some(ParagraphStyleId {
                    value: "Code".into(),
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut run1 = Run::default();
        run1.content.push(RunContent::Text(Text {
            text: "code1".into(),
            ..Default::default()
        }));
        para1.content.push(ParagraphContent::Run(run1));
        body.push(BodyContent::Paragraph(para1));

        // Normal paragraph (breaks the code block)
        let mut normal = Paragraph::default();
        let mut run_n = Run::default();
        run_n.content.push(RunContent::Text(Text {
            text: "normal text".into(),
            ..Default::default()
        }));
        normal.content.push(ParagraphContent::Run(run_n));
        body.push(BodyContent::Paragraph(normal));

        // Another code paragraph (separate block)
        let mut para2 = Paragraph {
            property: Some(ParagraphProperty {
                style_id: Some(ParagraphStyleId {
                    value: "Code".into(),
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let mut run2 = Run::default();
        run2.content.push(RunContent::Text(Text {
            text: "code2".into(),
            ..Default::default()
        }));
        para2.content.push(ParagraphContent::Run(run2));
        body.push(BodyContent::Paragraph(para2));

        let docx = rs_docx::Docx::default();
        let rels = HashMap::new();
        let mut numbering_resolver = NumberingResolver::new(&docx);
        let mut image_extractor = ImageExtractor::new_skip();
        let options = crate::ConvertOptions::default();
        let style_resolver = StyleResolver::new(&docx.styles);
        let mut context = ConversionContext::new(
            &rels,
            &mut numbering_resolver,
            &mut image_extractor,
            &options,
            None,
            None,
            None,
            &style_resolver,
        );

        let extractor = DocxExtractor;
        let doc = extractor.extract(&body, &mut context).unwrap();

        let code_blocks: Vec<_> = doc
            .blocks
            .iter()
            .filter(|b| match b {
                crate::core::ast::BlockNode::Paragraph(s)
                | crate::core::ast::BlockNode::TableHtml(s)
                | crate::core::ast::BlockNode::RawHtml(s) => s.contains("```"),
            })
            .collect();

        assert_eq!(
            code_blocks.len(),
            2,
            "Should have 2 separate code blocks, got: {:?}",
            doc.blocks
        );
    }

    #[test]
    fn test_consecutive_monospace_paragraphs_grouped_into_code_block() {
        use rs_docx::formatting::{CharacterProperty, Fonts};

        let mut body: Vec<BodyContent> = Vec::new();

        for line in ["use std::io;", "fn main() {", "}"] {
            let mut para = Paragraph::default();
            let run = Run {
                property: Some(CharacterProperty {
                    fonts: Some(Fonts::default().ascii("Courier New").h_ansi("Courier New")),
                    ..Default::default()
                }),
                content: vec![RunContent::Text(Text {
                    text: line.into(),
                    ..Default::default()
                })],
                ..Default::default()
            };
            para.content.push(ParagraphContent::Run(run));
            body.push(BodyContent::Paragraph(para));
        }

        let docx = rs_docx::Docx::default();
        let rels = HashMap::new();
        let mut nr = NumberingResolver::new(&docx);
        let mut ie = ImageExtractor::new_skip();
        let opts = crate::ConvertOptions::default();
        let sr = StyleResolver::new(&docx.styles);
        let mut ctx =
            ConversionContext::new(&rels, &mut nr, &mut ie, &opts, None, None, None, &sr);

        let doc = DocxExtractor.extract(&body, &mut ctx).unwrap();
        let combined = doc
            .blocks
            .iter()
            .map(|b| match b {
                crate::core::ast::BlockNode::Paragraph(s)
                | crate::core::ast::BlockNode::TableHtml(s)
                | crate::core::ast::BlockNode::RawHtml(s) => s.as_str(),
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        assert!(
            combined.contains("```"),
            "Monospace paragraphs should produce code block, got: {}",
            combined
        );
        assert!(
            combined.contains("use std::io;"),
            "Missing line 1 in: {}",
            combined
        );
        assert!(
            combined.contains("fn main() {"),
            "Missing line 2 in: {}",
            combined
        );
        // Should NOT have backtick-wrapped inline code
        assert!(
            !combined.contains("`use std::io;`"),
            "Should be code block, not inline code, got: {}",
            combined
        );
    }
}
