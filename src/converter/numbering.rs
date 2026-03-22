//! Numbering resolver - handles list numbering and indentation.

use rs_docx::Docx;
use std::collections::HashMap;

/// Resolver for DOCX numbering definitions.
pub struct NumberingResolver {
    /// Maps numId -> abstractNumId
    num_instances: HashMap<i32, i32>,
    /// Maps abstractNumId -> level definitions
    abstract_nums: HashMap<i32, Vec<LevelDef>>,
    /// Maps (numId, ilvl) -> startOverride value
    overrides: HashMap<(i32, i32), i32>,
    /// Maps abstractNumId -> current counters (one counter per level 0..9)
    /// Using abstractNumId allows continuous numbering even if numId changes (e.g. broken lists)
    /// None = uninitialized; Some(v) = current value (distinguishes 0 from unset).
    counters: HashMap<i32, Vec<Option<i32>>>,
    /// Maps abstractNumId -> base indentation level (shift)
    /// Used to normalize indentation for lists that start at high levels (e.g., Article at Level 4)
    level_shifts: HashMap<i32, i32>,
    /// Maps (numId, ilvl) -> override LevelDef (style change)
    style_overrides: HashMap<(i32, i32), LevelDef>,
}

#[derive(Clone, Debug)]
struct LevelDef {
    ilvl: i32,
    start: i32,
    num_fmt: String,
    lvl_text: Option<String>,
}

impl NumberingResolver {
    /// Creates a new numbering resolver from a parsed DOCX.
    pub fn new(docx: &Docx) -> Self {
        let mut num_instances = HashMap::new();
        let mut abstract_nums = HashMap::new();
        let mut overrides = HashMap::new();
        let mut style_overrides = HashMap::new();
        let mut level_shifts = HashMap::new();

        if let Some(numbering) = &docx.numbering {
            // Parse abstract numbering definitions
            for abs_num in &numbering.abstract_numberings {
                let abs_id = abs_num.abstract_num_id.map(|id| id as i32).unwrap_or(0);
                let mut levels = Vec::new();

                for lvl in &abs_num.levels {
                    let ilvl = lvl.i_level.map(|i| i as i32).unwrap_or(0);
                    let start = lvl
                        .start
                        .as_ref()
                        .and_then(|s| s.value)
                        .map(|v| v as i32)
                        .unwrap_or(1);
                    let num_fmt = lvl
                        .number_format
                        .as_ref()
                        .map(|f| f.value.to_string())
                        .unwrap_or_else(|| "decimal".to_string());
                    let lvl_text = lvl
                        .level_text
                        .as_ref()
                        .and_then(|t| t.value.as_ref())
                        .map(|v| v.to_string());

                    // Heuristic: If this level looks like an "Article" heading (제%1조),
                    // treat it as a base level (Level 0 equivalent) for indentation.
                    if let Some(text) = &lvl_text {
                        if text.contains("제") && text.contains("조") && text.contains("%") {
                            // Only set if not already set (prefer higher levels if multiple? No, prefer shallowest)
                            // But Article usually wraps everything.
                            level_shifts.entry(abs_id).or_insert(ilvl);
                        }
                    }

                    levels.push(LevelDef {
                        ilvl,
                        start,
                        num_fmt,
                        lvl_text,
                    });
                }

                levels.sort_by_key(|l| l.ilvl);
                abstract_nums.insert(abs_id, levels);
            }

            // Parse numbering instances
            for num in &numbering.numberings {
                if let (Some(num_id), Some(abs_ref)) = (num.num_id, &num.abstract_num_id) {
                    let nid = num_id as i32;
                    if let Some(abs_id) = abs_ref.value {
                        num_instances.insert(nid, abs_id as i32);
                    }

                    // Parse level overrides
                    for override_def in &num.level_overrides {
                        if let (Some(ilvl), Some(start_override)) =
                            (override_def.i_level, &override_def.start_override)
                        {
                            if let Some(val) = start_override.value {
                                overrides.insert((nid, ilvl as i32), val as i32);
                            }
                        }

                        // Parse level definition overrides (style change)
                        if let (Some(ilvl), Some(level)) =
                            (override_def.i_level, &override_def.level)
                        {
                            let ilvl = ilvl as i32;
                            let start = level
                                .start
                                .as_ref()
                                .and_then(|s| s.value)
                                .map(|v| v as i32)
                                .unwrap_or(1); // Default to 1 if not specified in override? Or inherit?
                                               // In strict XML, if start not present in override, it might inherit.
                                               // But here we construct a full LevelDef.
                                               // Ideally we should merge with abstract level.
                                               // For now, let's take what's in the override or default.

                            let num_fmt = level
                                .number_format
                                .as_ref()
                                .map(|f| f.value.to_string())
                                .unwrap_or_else(|| "decimal".to_string());
                            let lvl_text = level
                                .level_text
                                .as_ref()
                                .and_then(|t| t.value.as_ref())
                                .map(|v| v.to_string());

                            style_overrides.insert(
                                (nid, ilvl),
                                LevelDef {
                                    ilvl,
                                    start,
                                    num_fmt,
                                    lvl_text,
                                },
                            );
                        }
                    }
                }
            }
        }

        Self {
            num_instances,
            abstract_nums,
            overrides,
            style_overrides,
            counters: HashMap::new(),
            level_shifts,
        }
    }

    /// Gets the indentation level for a list item.
    pub fn get_indent(&self, num_id: i32, ilvl: i32) -> usize {
        let mut indent = ilvl;

        // Apply shift if exists for this abstract numbering
        if let Some(&abs_id) = self.num_instances.get(&num_id) {
            if let Some(&base_level) = self.level_shifts.get(&abs_id) {
                indent = indent.saturating_sub(base_level);
            }
        }

        indent as usize
    }

    /// Gets the marker for a list item (e.g., "1.", "-", "a)").
    /// Updates the internal counter state.
    pub fn next_marker(&mut self, num_id: i32, ilvl: i32) -> String {
        let Some(&abs_id) = self.num_instances.get(&num_id) else {
            return "-".to_string();
        };

        let Some(levels) = self.abstract_nums.get(&abs_id) else {
            return "-".to_string();
        };

        // Initialize counters for this abstract_num_id if not present
        // Use abstract_id as key to share state across different num_ids for same style
        let counters = self
            .counters
            .entry(abs_id)
            .or_insert_with(|| vec![None; 10]);

        // Find level definition
        // Check for style override first
        let level_def = self
            .style_overrides
            .get(&(num_id, ilvl))
            .or_else(|| levels.iter().find(|l| l.ilvl == ilvl))
            .or_else(|| levels.first());

        let Some(level) = level_def else {
            return "-".to_string();
        };

        // Increment current level
        let ilvl_idx = ilvl as usize;
        if counters.len() <= ilvl_idx {
            counters.resize(ilvl_idx + 1, None);
        }

        // Determine start value (check override for specific instance first)
        let override_start = self.overrides.get(&(num_id, ilvl)).copied();

        // Update logic:
        // None = uninitialized: initialize to start value (or override).
        // Some(v) = already running: increment.
        if counters[ilvl_idx].is_none() {
            counters[ilvl_idx] = Some(override_start.unwrap_or(level.start));
        } else {
            *counters[ilvl_idx].as_mut().unwrap() += 1;
        }

        // Reset lower levels
        for counter in counters.iter_mut().skip(ilvl_idx + 1) {
            *counter = None;
        }

        // Use level text if available (substituting placeholders)
        if let Some(text) = &level.lvl_text {
            let mut marker = text.clone();
            // Replace %1, %2, etc. with formatted numbers
            for (i, count) in counters.iter().enumerate() {
                let level_num = i + 1; // %1 is index 0
                let placeholder = format!("%{}", level_num);
                if marker.contains(&placeholder) {
                    // Find formatting for this level
                    let fmt = self
                        .style_overrides
                        .get(&(num_id, i as i32))
                        .map(|l| l.num_fmt.as_str())
                        .or_else(|| {
                            levels
                                .iter()
                                .find(|l| l.ilvl == i as i32)
                                .map(|l| l.num_fmt.as_str())
                        })
                        .unwrap_or("decimal");

                    // None means not yet initialized; treat as 1 for display purposes
                    let val = count.unwrap_or(1);

                    let formatted_num = Self::format_num(fmt, val);
                    marker = marker.replace(&placeholder, &formatted_num);
                }
            }
            return marker;
        }

        // Fallback: if no lvlText, add dot for standard types
        let raw_num = Self::format_num(&level.num_fmt, counters[ilvl_idx].unwrap_or(1));
        match level.num_fmt.as_str() {
            "decimal" | "lowerLetter" | "upperLetter" | "lowerRoman" | "upperRoman" => {
                format!("{}.", raw_num)
            }
            _ => raw_num,
        }
    }

    /// Formats a number according to the format string.
    fn format_num(fmt: &str, val: i32) -> String {
        match fmt {
            "bullet" | "none" => "-".to_string(),
            "decimal" => format!("{}", val),
            "lowerLetter" => {
                if (1..=26).contains(&val) {
                    char::from(b'a' + (val - 1) as u8).to_string()
                } else {
                    format!("{}", val)
                }
            }
            "upperLetter" => {
                if (1..=26).contains(&val) {
                    char::from(b'A' + (val - 1) as u8).to_string()
                } else {
                    format!("{}", val)
                }
            }
            "lowerRoman" => Self::to_roman(val).to_lowercase(),
            "upperRoman" => Self::to_roman(val),
            "koreanCounting" | "korean" | "ganada" => Self::format_ganada(val),
            "chosung" => Self::format_chosung(val),
            "geonodeo" => Self::format_geonodeo(val),
            "decimalEnclosedCircle" => Self::format_circle_number(val),
            _ => format!("{}", val),
        }
    }

    /// Converts a number to circled number (①②③...).
    fn format_circle_number(val: i32) -> String {
        // Unicode circled numbers: ① = U+2460, ② = U+2461, ... ⑳ = U+2473
        // Extended: ㉑ = U+3251, ㉒ = U+3252, ... ㊿ = U+32BF (21-50)
        if (1..=20).contains(&val) {
            char::from_u32(0x245F + val as u32)
                .map(|c| c.to_string())
                .unwrap_or_else(|| format!("{}", val))
        } else if (21..=50).contains(&val) {
            char::from_u32(0x3250 + (val - 20) as u32)
                .map(|c| c.to_string())
                .unwrap_or_else(|| format!("{}", val))
        } else {
            format!("{}", val) // Fallback for numbers outside supported range
        }
    }

    /// Indexes into a char array by 1-based `val`, falling back to the numeric string.
    fn format_from_chars(chars: &[char], val: i32) -> String {
        if val >= 1 && (val as usize) <= chars.len() {
            chars[(val - 1) as usize].to_string()
        } else {
            format!("{}", val)
        }
    }

    /// Converts a number to Korean Ganada (가, 나, 다...).
    fn format_ganada(val: i32) -> String {
        const CHARS: &[char] = &[
            '가', '나', '다', '라', '마', '바', '사', '아', '자', '차', '카', '타', '파', '하',
        ];
        Self::format_from_chars(CHARS, val)
    }

    /// Converts a number to Korean Geonodeo (거, 너, 더...).
    fn format_geonodeo(val: i32) -> String {
        const CHARS: &[char] = &[
            '거', '너', '더', '러', '머', '버', '서', '어', '저', '처', '커', '터', '퍼', '허',
        ];
        Self::format_from_chars(CHARS, val)
    }

    /// Converts a number to Korean Chosung (ㄱ, ㄴ, ㄷ...).
    fn format_chosung(val: i32) -> String {
        const CHARS: &[char] = &[
            'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
        ];
        Self::format_from_chars(CHARS, val)
    }

    /// Converts a number to Roman numeral.
    fn to_roman(mut num: i32) -> String {
        const ROMAN_TABLE: &[(i32, &str)] = &[
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        if num <= 0 {
            return num.to_string();
        }
        let mut result = String::new();
        for &(v, s) in ROMAN_TABLE {
            while num >= v {
                result.push_str(s);
                num -= v;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rs_docx::document::{
        AbstractNum, AbstractNumId, Level, LevelOverride, LevelStart, LevelText, Num, NumFmt,
        Numbering,
    };
    use std::borrow::Cow;

    #[test]
    fn test_counter_with_start_zero_increments() {
        // A list with start=0 should produce 0, 1, 2, not 0, 0, 0
        let abstract_num = AbstractNum {
            abstract_num_id: Some(1),
            levels: vec![Level {
                i_level: Some(0),
                start: Some(LevelStart { value: Some(0) }),
                number_format: Some(NumFmt {
                    value: Cow::Borrowed("decimal"),
                }),
                level_text: Some(LevelText {
                    value: Some(Cow::Borrowed("%1.")),
                }),
                ..Default::default()
            }],
            ..Default::default()
        };
        let num = Num {
            num_id: Some(1),
            abstract_num_id: Some(AbstractNumId { value: Some(1) }),
            ..Default::default()
        };
        let numbering = Numbering {
            abstract_numberings: vec![abstract_num],
            numberings: vec![num],
        };
        let docx = Docx {
            numbering: Some(numbering),
            ..Default::default()
        };
        let mut resolver = NumberingResolver::new(&docx);

        let m1 = resolver.next_marker(1, 0);
        let m2 = resolver.next_marker(1, 0);
        let m3 = resolver.next_marker(1, 0);

        assert_eq!(m1, "0.");
        assert_eq!(m2, "1.");
        assert_eq!(m3, "2.");
    }

    // --- format_num direct tests ---

    #[test]
    fn test_format_num_decimal() {
        assert_eq!(NumberingResolver::format_num("decimal", 5), "5");
    }

    #[test]
    fn test_format_num_decimal_zero() {
        assert_eq!(NumberingResolver::format_num("decimal", 0), "0");
    }

    #[test]
    fn test_format_num_lower_letter() {
        assert_eq!(NumberingResolver::format_num("lowerLetter", 1), "a");
    }

    #[test]
    fn test_format_num_lower_letter_z() {
        assert_eq!(NumberingResolver::format_num("lowerLetter", 26), "z");
    }

    #[test]
    fn test_format_num_lower_letter_overflow() {
        assert_eq!(NumberingResolver::format_num("lowerLetter", 27), "27");
    }

    #[test]
    fn test_format_num_upper_letter() {
        assert_eq!(NumberingResolver::format_num("upperLetter", 3), "C");
    }

    #[test]
    fn test_format_num_bullet() {
        assert_eq!(NumberingResolver::format_num("bullet", 1), "-");
    }

    #[test]
    fn test_format_num_none() {
        assert_eq!(NumberingResolver::format_num("none", 1), "-");
    }

    #[test]
    fn test_format_num_unknown() {
        assert_eq!(NumberingResolver::format_num("unknownFormat", 5), "5");
    }

    // --- to_roman tests ---

    #[test]
    fn test_to_roman_1() {
        assert_eq!(NumberingResolver::to_roman(1), "I");
    }

    #[test]
    fn test_to_roman_4() {
        assert_eq!(NumberingResolver::to_roman(4), "IV");
    }

    #[test]
    fn test_to_roman_9() {
        assert_eq!(NumberingResolver::to_roman(9), "IX");
    }

    #[test]
    fn test_to_roman_zero() {
        assert_eq!(NumberingResolver::to_roman(0), "0");
    }

    #[test]
    fn test_format_num_lower_roman() {
        assert_eq!(NumberingResolver::format_num("lowerRoman", 4), "iv");
    }

    #[test]
    fn test_format_num_upper_roman() {
        assert_eq!(NumberingResolver::format_num("upperRoman", 4), "IV");
    }

    // --- Korean format tests ---

    #[test]
    fn test_format_ganada() {
        assert_eq!(NumberingResolver::format_ganada(1), "가");
        assert_eq!(NumberingResolver::format_ganada(2), "나");
        assert_eq!(NumberingResolver::format_ganada(3), "다");
    }

    #[test]
    fn test_format_ganada_overflow() {
        assert_eq!(NumberingResolver::format_ganada(15), "15");
    }

    #[test]
    fn test_format_chosung() {
        assert_eq!(NumberingResolver::format_chosung(1), "ㄱ");
        assert_eq!(NumberingResolver::format_chosung(2), "ㄴ");
    }

    #[test]
    fn test_format_geonodeo() {
        assert_eq!(NumberingResolver::format_geonodeo(1), "거");
        assert_eq!(NumberingResolver::format_geonodeo(2), "너");
    }

    // --- Circle number tests ---

    #[test]
    fn test_format_circle_1() {
        assert_eq!(NumberingResolver::format_circle_number(1), "①");
    }

    #[test]
    fn test_format_circle_20() {
        assert_eq!(NumberingResolver::format_circle_number(20), "⑳");
    }

    #[test]
    fn test_format_circle_21() {
        // U+3251 = ㉑
        assert_eq!(NumberingResolver::format_circle_number(21), "㉑");
    }

    #[test]
    fn test_format_circle_overflow() {
        assert_eq!(NumberingResolver::format_circle_number(51), "51");
    }

    // --- Existing integration tests ---

    #[test]
    fn test_lvl_override_style_change() {
        // Construct AbstractNum: Level 0 is decimal "%1."
        let abstract_num = AbstractNum {
            abstract_num_id: Some(1),
            levels: vec![Level {
                i_level: Some(0),
                start: Some(LevelStart { value: Some(1) }),
                number_format: Some(NumFmt {
                    value: Cow::Borrowed("decimal"),
                }),
                level_text: Some(LevelText {
                    value: Some(Cow::Borrowed("%1.")),
                }),
                ..Default::default()
            }],
            ..Default::default()
        };

        // Construct Num: References AbstractNum 1, but overrides Level 0 to upperLetter "%1)"
        let num = Num {
            num_id: Some(2),
            abstract_num_id: Some(AbstractNumId { value: Some(1) }),
            level_overrides: vec![LevelOverride {
                i_level: Some(0),
                start_override: None,
                level: Some(Level {
                    i_level: Some(0),
                    start: Some(LevelStart { value: Some(1) }),
                    number_format: Some(NumFmt {
                        value: Cow::Borrowed("upperLetter"),
                    }),
                    level_text: Some(LevelText {
                        value: Some(Cow::Borrowed("%1)")),
                    }),
                    ..Default::default()
                }),
            }],
        };

        let numbering = Numbering {
            abstract_numberings: vec![abstract_num],
            numberings: vec![num],
        };

        let docx = Docx {
            numbering: Some(numbering),
            ..Default::default()
        };

        let mut resolver = NumberingResolver::new(&docx);

        // numId 2, defaults to decimal, but overridden to upperLetter
        let marker = resolver.next_marker(2, 0);
        assert_eq!(marker, "A)");
    }
}
