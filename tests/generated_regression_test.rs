use undocx::{ConvertOptions, DocxToMarkdown};
use rs_docx::document::{
    AbstractNum, AbstractNumId, BodyContent, Comment, CommentReference, Comments, FootNote,
    FootNotes, FootnoteReference, Level, LevelStart, LevelText, Num, NumFmt, Numbering, Paragraph,
    Run, RunContent,
};
use rs_docx::formatting::{NumberingProperty, ParagraphProperty};
use rs_docx::Docx;
use std::borrow::Cow;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_docx_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after UNIX_EPOCH")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "undocx_regression_{}_{}_{}.docx",
        prefix,
        std::process::id(),
        nanos
    ))
}

#[test]
fn generated_docx_deep_list_indentation_regression() {
    let mut docx = Docx {
        numbering: Some(Numbering {
            abstract_numberings: vec![AbstractNum {
                abstract_num_id: Some(1),
                levels: vec![Level {
                    i_level: Some(3),
                    start: Some(LevelStart { value: Some(1) }),
                    number_format: Some(NumFmt {
                        value: Cow::Borrowed("decimal"),
                    }),
                    level_text: Some(LevelText {
                        value: Some(Cow::Borrowed("%4.")),
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            numberings: vec![Num {
                num_id: Some(2),
                abstract_num_id: Some(AbstractNumId { value: Some(1) }),
                ..Default::default()
            }],
        }),
        ..Default::default()
    };

    let para = Paragraph::default()
        .property(ParagraphProperty {
            numbering: Some(NumberingProperty::from((2isize, 3isize))),
            ..Default::default()
        })
        .push_text("Deep Item");
    docx.document.push(para);

    let path = temp_docx_path("deep_list");
    docx.write_file(&path)
        .expect("failed to write generated docx");

    let converter = DocxToMarkdown::new(ConvertOptions::default());
    let markdown = converter
        .convert(&path)
        .expect("failed to convert generated deep-list docx");

    let _ = std::fs::remove_file(&path);

    let first_line = markdown.lines().next().unwrap_or("");
    assert_eq!(first_line, "      1. Deep Item");
}

#[test]
fn generated_docx_duplicate_note_comment_references_regression() {
    let footnote_body = Paragraph::default().push_text("Same footnote text");
    let comment_body = Paragraph::default().push_text("Shared comment");
    let mut docx = Docx {
        footnotes: Some(FootNotes {
            content: vec![FootNote {
                id: Some(5),
                content: vec![BodyContent::Paragraph(footnote_body)],
                ..Default::default()
            }],
        }),
        comments: Some(Comments {
            comments: vec![Comment {
                id: Some(9),
                author: Cow::Borrowed("tester"),
                content: comment_body,
            }],
        }),
        ..Default::default()
    };

    let footnote_refs = Paragraph::default()
        .push(
            Run::default().push(RunContent::FootnoteReference(FootnoteReference {
                id: Some(Cow::Borrowed("5")),
                ..Default::default()
            })),
        )
        .push(
            Run::default().push(RunContent::FootnoteReference(FootnoteReference {
                id: Some(Cow::Borrowed("5")),
                ..Default::default()
            })),
        );
    docx.document.push(footnote_refs);

    let comment_refs = Paragraph::default()
        .push(
            Run::default().push(RunContent::CommentReference(CommentReference {
                id: Some(Cow::Borrowed("9")),
            })),
        )
        .push(
            Run::default().push(RunContent::CommentReference(CommentReference {
                id: Some(Cow::Borrowed("9")),
            })),
        );
    docx.document.push(comment_refs);

    let path = temp_docx_path("notes_comments");
    docx.write_file(&path)
        .expect("failed to write generated docx");

    let converter = DocxToMarkdown::new(ConvertOptions::default());
    let markdown = converter
        .convert(&path)
        .expect("failed to convert generated note/comment docx");

    let _ = std::fs::remove_file(&path);

    assert!(markdown.contains("[^1][^1]"));
    assert!(markdown.contains("[^c9][^c9]"));
    assert_eq!(markdown.matches("[^1]:").count(), 1);
    assert_eq!(markdown.matches("[^c9]:").count(), 1);
    assert!(markdown.contains("[^1]: Same footnote text"));
    assert!(markdown.contains("[^c9]: Shared comment"));
}
