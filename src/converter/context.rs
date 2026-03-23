use super::{ImageExtractor, NumberingResolver, StyleResolver};
use crate::core::ast::ReferenceDefinitions;
use crate::{ConvertOptions, Result};
use std::collections::{HashMap, HashSet};

/// Shared mutable state threaded through the conversion pipeline.
///
/// Carries references to the document's relationships, numbering definitions,
/// images, styles, and accumulates footnote / endnote / comment definitions
/// encountered during extraction. Custom [`AstExtractor`](crate::adapters::docx::AstExtractor)
/// implementations receive this context to register references and resolve
/// document resources.
pub struct ConversionContext<'a> {
    rels: &'a HashMap<String, String>,
    numbering: &'a mut NumberingResolver,
    image_extractor: &'a mut ImageExtractor,
    options: &'a ConvertOptions,
    style_resolver: &'a StyleResolver<'a>,
    footnotes: Vec<String>,
    footnote_index_by_id: HashMap<isize, usize>,
    footnote_text_by_id: HashMap<isize, String>,
    endnotes: Vec<String>,
    endnote_index_by_id: HashMap<isize, usize>,
    endnote_text_by_id: HashMap<isize, String>,
    comments: Vec<(String, String)>,
    seen_comment_ids: HashSet<String>,
    comment_text_by_id: HashMap<String, String>,
    missing_references: Vec<String>,
    in_table_cell: bool,
}

impl<'a> ConversionContext<'a> {
    /// Creates a new context from the parsed DOCX components.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rels: &'a HashMap<String, String>,
        numbering: &'a mut NumberingResolver,
        image_extractor: &'a mut ImageExtractor,
        options: &'a ConvertOptions,
        docx_comments: Option<&'a rs_docx::document::Comments<'a>>,
        docx_footnotes: Option<&'a rs_docx::document::FootNotes<'a>>,
        docx_endnotes: Option<&'a rs_docx::document::EndNotes<'a>>,
        style_resolver: &'a StyleResolver<'a>,
    ) -> Self {
        let comment_text_by_id = docx_comments
            .map(|comments| {
                comments
                    .comments
                    .iter()
                    .filter_map(|comment| {
                        comment
                            .id
                            .map(|id| (id.to_string(), comment.content.text().to_string()))
                    })
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        let footnote_text_by_id = docx_footnotes
            .map(|footnotes| {
                footnotes
                    .content
                    .iter()
                    .filter_map(|footnote| {
                        footnote.id.map(|id| {
                            let text = footnote
                                .content
                                .iter()
                                .filter_map(|bc| match bc {
                                    rs_docx::document::BodyContent::Paragraph(p) => {
                                        Some(p.text().to_string())
                                    }
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                                .join(" ");
                            (id, text)
                        })
                    })
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        let endnote_text_by_id = docx_endnotes
            .map(|endnotes| {
                endnotes
                    .content
                    .iter()
                    .filter_map(|endnote| {
                        endnote.id.map(|id| {
                            let text = endnote
                                .content
                                .iter()
                                .filter_map(|bc| match bc {
                                    rs_docx::document::BodyContent::Paragraph(p) => {
                                        Some(p.text().to_string())
                                    }
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                                .join(" ");
                            (id, text)
                        })
                    })
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        Self {
            rels,
            numbering,
            image_extractor,
            options,
            style_resolver,
            footnotes: Vec::new(),
            footnote_index_by_id: HashMap::new(),
            footnote_text_by_id,
            endnotes: Vec::new(),
            endnote_index_by_id: HashMap::new(),
            endnote_text_by_id,
            comments: Vec::new(),
            seen_comment_ids: HashSet::new(),
            comment_text_by_id,
            missing_references: Vec::new(),
            in_table_cell: false,
        }
    }

    // ── Table cell state ────────────────────────────────────────────

    /// Marks whether conversion is currently inside a table cell.
    pub fn set_in_table_cell(&mut self, value: bool) {
        self.in_table_cell = value;
    }

    /// Returns `true` if conversion is currently inside a table cell.
    pub fn is_in_table_cell(&self) -> bool {
        self.in_table_cell
    }

    // ── Reference registration ──────────────────────────────────────

    /// Registers a comment reference and returns the Markdown footnote marker
    /// (e.g., `[^c3]`). Duplicate calls with the same `id` return the same marker.
    pub fn register_comment_reference(&mut self, id: &str) -> String {
        if !self.seen_comment_ids.contains(id) {
            let comment_text = self.comment_text_by_id.get(id).cloned().unwrap_or_else(|| {
                self.missing_references.push(format!("comment:{id}"));
                String::new()
            });

            self.comments.push((id.to_string(), comment_text));
            self.seen_comment_ids.insert(id.to_string());
        }

        format!("[^c{}]", id)
    }

    /// Registers a footnote reference and returns the Markdown footnote marker
    /// (e.g., `[^1]`). Duplicate calls with the same `id` return the same marker.
    pub fn register_footnote_reference(&mut self, id: isize) -> String {
        if let Some(idx) = self.footnote_index_by_id.get(&id).copied() {
            return format!("[^{}]", idx);
        }

        let footnote_text = self
            .footnote_text_by_id
            .get(&id)
            .cloned()
            .unwrap_or_else(|| {
                self.missing_references.push(format!("footnote:{id}"));
                String::new()
            });

        self.footnotes.push(footnote_text);
        let idx = self.footnotes.len();
        self.footnote_index_by_id.insert(id, idx);

        format!("[^{}]", idx)
    }

    /// Registers an endnote reference and returns the Markdown footnote marker
    /// (e.g., `[^en1]`). Duplicate calls with the same `id` return the same marker.
    pub fn register_endnote_reference(&mut self, id: isize) -> String {
        if let Some(idx) = self.endnote_index_by_id.get(&id).copied() {
            return format!("[^en{}]", idx);
        }

        let endnote_text = self
            .endnote_text_by_id
            .get(&id)
            .cloned()
            .unwrap_or_else(|| {
                self.missing_references.push(format!("endnote:{id}"));
                String::new()
            });

        self.endnotes.push(endnote_text);
        let idx = self.endnotes.len();
        self.endnote_index_by_id.insert(id, idx);

        format!("[^en{}]", idx)
    }

    // ── Reference output ────────────────────────────────────────────

    /// Builds the collected [`ReferenceDefinitions`] for the final document.
    pub fn reference_definitions(&self) -> ReferenceDefinitions {
        ReferenceDefinitions {
            footnotes: self.footnotes.clone(),
            endnotes: self.endnotes.clone(),
            comments: self.comments.clone(),
        }
    }

    /// Drains and returns any reference IDs that could not be resolved.
    ///
    /// Used by [`DocxToMarkdown`](crate::DocxToMarkdown) when strict reference
    /// validation is enabled.
    pub fn take_missing_references(&mut self) -> Vec<String> {
        std::mem::take(&mut self.missing_references)
    }

    // ── Document resource access ────────────────────────────────────

    /// Looks up a relationship target (e.g., hyperlink URL) by its ID.
    pub fn relationship_target(&self, id: &str) -> Option<&str> {
        self.rels.get(id).map(String::as_str)
    }

    /// Extracts an image from a `<w:drawing>` element using the configured
    /// [`ImageHandling`](crate::ImageHandling) strategy.
    pub fn extract_image_from_drawing(
        &mut self,
        drawing: &rs_docx::document::Drawing,
    ) -> Result<Option<String>> {
        self.image_extractor
            .extract_from_drawing(drawing, self.rels)
    }

    /// Extracts an image from a `<w:pict>` (VML) element using the configured
    /// [`ImageHandling`](crate::ImageHandling) strategy.
    pub fn extract_image_from_pict(
        &mut self,
        pict: &rs_docx::document::Pict,
    ) -> Result<Option<String>> {
        self.image_extractor.extract_from_pict(pict, self.rels)
    }

    // ── Style resolution ────────────────────────────────────────────

    /// Resolves the effective character properties for a run, merging direct
    /// properties with inherited style chain values.
    pub fn resolve_run_property(
        &self,
        direct_props: Option<&rs_docx::formatting::CharacterProperty<'a>>,
        run_style_id: Option<&str>,
        para_style_id: Option<&str>,
    ) -> rs_docx::formatting::CharacterProperty<'a> {
        self.style_resolver
            .resolve_run_property(direct_props, run_style_id, para_style_id)
    }

    /// Resolves the effective paragraph properties, merging direct properties
    /// with inherited style chain values.
    pub fn resolve_paragraph_property(
        &self,
        direct_props: Option<&rs_docx::formatting::ParagraphProperty<'a>>,
        para_style_id: Option<&str>,
    ) -> rs_docx::formatting::ParagraphProperty<'a> {
        self.style_resolver
            .resolve_paragraph_property(direct_props, para_style_id)
    }

    // ── List numbering ──────────────────────────────────────────────

    /// Returns the next list marker for the given numbering definition and level
    /// (e.g., `"1."`, `"-"`, `"가."`, `"(a)"`).
    pub fn next_list_marker(&mut self, num_id: i32, ilvl: i32) -> String {
        self.numbering.next_marker(num_id, ilvl)
    }

    /// Returns the indentation level for the given numbering definition and level.
    pub fn list_indent_level(&self, num_id: i32, ilvl: i32) -> usize {
        self.numbering.get_indent(num_id, ilvl)
    }

    // ── Option accessors ────────────────────────────────────────────

    /// Whether the [`preserve_whitespace`](crate::ConvertOptions::preserve_whitespace)
    /// option is enabled.
    pub fn preserve_whitespace(&self) -> bool {
        self.options.preserve_whitespace
    }

    /// Whether HTML `<u>` tags should be emitted for underlined text.
    pub fn html_underline_enabled(&self) -> bool {
        self.options.html_underline
    }

    /// Whether HTML `<s>` tags should be emitted for strikethrough text.
    pub fn html_strikethrough_enabled(&self) -> bool {
        self.options.html_strikethrough
    }

    // ── Counters ────────────────────────────────────────────────────

    /// Number of footnotes registered so far.
    pub fn footnote_count(&self) -> usize {
        self.footnotes.len()
    }

    /// Number of endnotes registered so far.
    pub fn endnote_count(&self) -> usize {
        self.endnotes.len()
    }

    /// Number of comments registered so far.
    pub fn comment_count(&self) -> usize {
        self.comments.len()
    }

    /// Returns the comment `(id, text)` at the given index, if it exists.
    pub fn comment_at(&self, index: usize) -> Option<(&str, &str)> {
        self.comments
            .get(index)
            .map(|(id, text)| (id.as_str(), text.as_str()))
    }
}
