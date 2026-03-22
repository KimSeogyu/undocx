use undocx::{ConvertOptions, DocxToMarkdown, Error};
use rs_docx::document::{
    BodyContent, Comment, CommentReference, Comments, EndNote, EndNotes, EndnoteReference,
    FootNote, FootNotes, FootnoteReference, Paragraph, Run, RunContent,
};
use rs_docx::Docx;
use std::borrow::Cow;
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }

    fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) == 1
    }

    fn next_inclusive_i32(&mut self, low: i32, high: i32) -> i32 {
        let span = (high - low + 1) as u64;
        low + (self.next_u64() % span) as i32
    }
}

fn temp_docx_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after UNIX_EPOCH")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "undocx_invariant_{}_{}_{}.docx",
        prefix,
        std::process::id(),
        nanos
    ))
}

fn mk_comment(id: i32, text: String) -> Comment<'static> {
    Comment {
        id: Some(id as isize),
        author: Cow::Borrowed("invariant"),
        content: Paragraph::default().push_text(text),
    }
}

fn mk_footnote(id: i32, text: String) -> FootNote<'static> {
    FootNote {
        id: Some(id as isize),
        content: vec![BodyContent::Paragraph(Paragraph::default().push_text(text))],
        ..Default::default()
    }
}

fn mk_endnote(id: i32, text: String) -> EndNote<'static> {
    EndNote {
        id: Some(id as isize),
        content: vec![BodyContent::Paragraph(Paragraph::default().push_text(text))],
        ..Default::default()
    }
}

#[test]
fn randomized_reference_validation_invariants() {
    let mut rng = Lcg::new(0xD02C_2026_0214);
    let iterations = 24;

    for i in 0..iterations {
        let mut available_footnotes = HashSet::new();
        let mut available_comments = HashSet::new();
        let mut available_endnotes = HashSet::new();

        for id in 1..=3 {
            if rng.next_bool() {
                available_footnotes.insert(id);
            }
            if rng.next_bool() {
                available_comments.insert(id);
            }
            if rng.next_bool() {
                available_endnotes.insert(id);
            }
        }

        let mut docx = Docx::default();
        if !available_footnotes.is_empty() {
            let mut content = available_footnotes
                .iter()
                .map(|id| mk_footnote(*id, format!("fn-{}", id)))
                .collect::<Vec<_>>();
            content.sort_by_key(|n| n.id);
            docx.footnotes = Some(FootNotes { content });
        }
        if !available_comments.is_empty() {
            let mut comments = available_comments
                .iter()
                .map(|id| mk_comment(*id, format!("comment-{}", id)))
                .collect::<Vec<_>>();
            comments.sort_by_key(|n| n.id);
            docx.comments = Some(Comments { comments });
        }
        if !available_endnotes.is_empty() {
            let mut content = available_endnotes
                .iter()
                .map(|id| mk_endnote(*id, format!("en-{}", id)))
                .collect::<Vec<_>>();
            content.sort_by_key(|n| n.id);
            docx.endnotes = Some(EndNotes { content });
        }

        let mut expected_missing = false;
        for p in 0..5 {
            let mut para = Paragraph::default().push_text(format!("case-{}-p{}", i, p));

            let fn_id = rng.next_inclusive_i32(1, 4);
            let comment_id = rng.next_inclusive_i32(1, 4);
            let en_id = rng.next_inclusive_i32(1, 4);

            if rng.next_bool() {
                if !available_footnotes.contains(&fn_id) {
                    expected_missing = true;
                }
                para = para.push(Run::default().push(RunContent::FootnoteReference(
                    FootnoteReference {
                        id: Some(Cow::Owned(fn_id.to_string())),
                        ..Default::default()
                    },
                )));
            }

            if rng.next_bool() {
                if !available_comments.contains(&comment_id) {
                    expected_missing = true;
                }
                para = para.push(Run::default().push(RunContent::CommentReference(
                    CommentReference {
                        id: Some(Cow::Owned(comment_id.to_string())),
                    },
                )));
            }

            if rng.next_bool() {
                if !available_endnotes.contains(&en_id) {
                    expected_missing = true;
                }
                para = para.push(Run::default().push(RunContent::EndnoteReference(
                    EndnoteReference {
                        id: Some(Cow::Owned(en_id.to_string())),
                        ..Default::default()
                    },
                )));
            }

            docx.document.push(para);
        }

        let path = temp_docx_path(&format!("case_{}", i));
        docx.write_file(&path)
            .expect("failed to write invariant test docx");

        let converter_relaxed = DocxToMarkdown::new(ConvertOptions::default());
        let relaxed = converter_relaxed.convert(&path);
        assert!(
            relaxed.is_ok(),
            "relaxed conversion must not fail on iteration {}: {:?}",
            i,
            relaxed.err()
        );

        let strict_converter = DocxToMarkdown::new(ConvertOptions {
            strict_reference_validation: true,
            ..Default::default()
        });
        let strict = strict_converter.convert(&path);
        let _ = std::fs::remove_file(&path);

        match (expected_missing, strict) {
            (true, Err(Error::MissingReference(_))) => {}
            (true, other) => panic!(
                "expected MissingReference on iteration {i}, got {:?}",
                other
            ),
            (false, Ok(_)) => {}
            (false, other) => panic!("expected strict success on iteration {i}, got {:?}", other),
        }
    }
}
