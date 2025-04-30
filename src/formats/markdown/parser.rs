use crate::formats::markdown;

use super::{Node, Nodes};

#[derive(Debug)]
pub enum ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Markdown parse error")
    }
}

impl std::error::Error for ParseError {}

fn is_tag_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_' || c == '/'
}

fn count_leading_chars(s: &str, c: char) -> usize {
    s.chars().take_while(|&x| x == c).count()
}

fn split_string_at_optional_pipe(s: &str) -> (&str, Option<&str>) {
    let mut split = s.splitn(2, '|');
    let first = split.next().unwrap();
    let second = split.next();
    (first, second)
}

fn split_task_string(s: &str) -> (Option<markdown::TaskStatus>, String) {
    // Check for index 0 and 2 being [ and ] respectively

    if s.chars().nth(0) == Some('[') && s.chars().nth(2) == Some(']') {
        let status = match s.chars().nth(1) {
            Some(' ') => Some(markdown::TaskStatus::Pending),
            _ => Some(markdown::TaskStatus::Completed),
        };

        let text = s.chars().skip(3).collect();

        (status, text)
    } else {
        (None, s.to_string())
    }
}

pub enum PreParsed {
    Parsed(Box<markdown::Block>),
    Unparsed(Box<Vec<String>>),
}

struct ParseLink {
    current: Box<Node>,
    next: Option<Box<ParseLink>>,
}

impl ParseLink {
    fn first(node: Node) -> ParseLink {
        ParseLink {
            current: Box::new(node),
            next: None,
        }
    }

    fn adjoin(node: Node, next: ParseLink) -> ParseLink {
        ParseLink {
            current: Box::new(node),
            next: Some(Box::new(next)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct ParagraphFlags {
    bold: bool,
    italic: bool,
    link: bool,
}

impl ParagraphFlags {
    fn new() -> ParagraphFlags {
        ParagraphFlags {
            bold: false,
            italic: false,
            link: false,
        }
    }

    fn with_bold(&self) -> ParagraphFlags {
        ParagraphFlags {
            bold: true,
            ..*self
        }
    }

    fn with_italic(&self) -> ParagraphFlags {
        ParagraphFlags {
            italic: true,
            ..*self
        }
    }

    fn with_link(&self) -> ParagraphFlags {
        ParagraphFlags {
            link: true,
            ..*self
        }
    }
}

struct ParseReturn(Node, usize);

struct LittleParser {
    parser: fn(&ParagraphParser, usize, ParagraphFlags) -> Option<ParseReturn>,
    index: usize,
}

impl LittleParser {
    fn new(
        parser: fn(&ParagraphParser, usize, ParagraphFlags) -> Option<ParseReturn>,
        index: usize,
    ) -> LittleParser {
        LittleParser { parser, index }
    }

    fn parse(&self, parser: &ParagraphParser, flags: ParagraphFlags) -> Option<ParseReturn> {
        (self.parser)(parser, self.index, flags)
    }
}

struct ParagraphParser {
    chars: Vec<char>,
}

impl ParagraphParser {
    pub fn for_string<S: ToString>(string: S) -> ParagraphParser {
        let chars: Vec<char> = string.to_string().chars().collect();

        ParagraphParser { chars }
    }

    fn at_end(&self, index: usize) -> bool {
        index >= self.chars.len()
    }

    fn at(&self, index: usize) -> Option<char> {
        self.chars.get(index).cloned()
    }

    fn check_at(&self, index: usize, text: &str) -> (bool, usize) {
        let mut i = 0;
        for c in text.chars() {
            if self.at(index + i) != Some(c) {
                return (false, index + i);
            }
            i += 1;
        }
        (true, index + i)
    }

    fn parse_bold_extra_wrap(
        &self,
        nodes: Nodes,
        i: usize,
    ) -> Option<ParseReturn> {
        Some(ParseReturn(Node::Bold(Box::new(Node::Nodes(nodes))), i))
    }

    fn parse_bold(&self, index: usize, flags: ParagraphFlags) -> Option<ParseReturn> {
        self.parse_recursively(
            index,
            |parser, i| parser.check_at(i, "**"),
            ParagraphParser::parse_bold_extra_wrap,
            flags.with_bold(),
        )
    }

    fn parse_italic_extra_wrap(
        &self,
        nodes: Nodes,
        i: usize,
    ) -> Option<ParseReturn> {
        Some(ParseReturn(Node::Italic(Box::new(Node::Nodes(nodes))), i))
    }

    fn parse_italic(&self, index: usize, flags: ParagraphFlags) -> Option<ParseReturn> {
        self.parse_recursively(
            index,
            |parser, i| parser.check_at(i, "*"),
            ParagraphParser::parse_italic_extra_wrap,
            flags.with_italic(),
        )
    }

    fn parse_tag(
        &self,
        index: usize,
        flags: ParagraphFlags,
    ) -> Option<ParseReturn> {
        let mut current = String::new();
        let mut i = index;

        // Parse while we have a tag character (`is_tag_char`)

        while !self.at_end(i) {
            if !is_tag_char(self.at(i).unwrap()) {
                
                return if current.is_empty() {
                    None
                } else {
                    Some(ParseReturn(Node::Tag(current), i))
                };
            }

            current.push(self.at(i).unwrap());
            i += 1;
        }

        if current.is_empty() {
            None
        } else {
            Some(ParseReturn(Node::Tag(current), i))
        }
    }

    fn try_find_parsers(&self, index: usize, flags: ParagraphFlags) -> Vec<LittleParser> {
        fn find_bold(
            parser: &ParagraphParser,
            index: usize,
            flags: ParagraphFlags,
        ) -> Option<LittleParser> {
            if flags.bold {
                return None;
            }

            if let (true, new_i) = parser.check_at(index, "**") {
                Some(LittleParser::new(ParagraphParser::parse_bold, new_i))
            } else {
                None
            }
        }

        fn find_italic(
            parser: &ParagraphParser,
            index: usize,
            flags: ParagraphFlags,
        ) -> Option<LittleParser> {
            if flags.italic {
                return None;
            }

            if let (true, new_i) = parser.check_at(index, "*") {
                Some(LittleParser::new(ParagraphParser::parse_italic, new_i))
            } else {
                None
            }
        }

        fn find_tag(
            parser: &ParagraphParser,
            index: usize,
            flags: ParagraphFlags,
        ) -> Option<LittleParser> {
            if let (true, new_i) = parser.check_at(index, "#") {
                Some(LittleParser::new(ParagraphParser::parse_tag, new_i))
            } else {
                None
            }
        }

        let mut parsers = Vec::new();

        if let Some(parser) = find_bold(self, index, flags) {
            parsers.push(parser);
        }

        if let Some(parser) = find_italic(self, index, flags) {
            parsers.push(parser);
        }

        if let Some(parser) = find_tag(self, index, flags) {
            parsers.push(parser);
        }

        parsers
    }

    fn try_run_parsers(&self, i: usize, flags: ParagraphFlags) -> Option<ParseReturn> {
        for parser in self.try_find_parsers(i, flags) {
            if let Some(parse_return) = parser.parse(self, flags) {
                return Some(parse_return);
            }
        }
        None
    }

    fn parse_recursively(
        &self,
        mut i: usize,
        extra_end_condition: fn(&ParagraphParser, usize) -> (bool, usize),
        extra_wrap: fn(&ParagraphParser, Nodes, usize) -> Option<ParseReturn>,
        flags: ParagraphFlags,
    ) -> Option<ParseReturn> {
        let mut nodes = Vec::new();
        let mut current_string = String::new();

        while !self.at_end(i) {
            if let (true, new_i) = extra_end_condition(self, i) {
                i = new_i;
                break;
            }

            if let Some(ParseReturn(new_node, new_i)) = self.try_run_parsers(i, flags) {
                if !current_string.is_empty() {
                    nodes.push(Node::Text(current_string));
                    current_string = String::new();
                }
                nodes.push(new_node);
                i = new_i;
            } else {
                current_string.push(self.at(i).unwrap());
                i += 1;
            }
        }

        if !current_string.is_empty() {
            nodes.push(Node::Text(current_string));
        }

        extra_wrap(self, nodes, i)
    }

    fn no_extra_end_condition(_parser: &ParagraphParser, i: usize) -> (bool, usize) {
        (false, i)
    }

    fn no_extra_wrap(
        _parser: &ParagraphParser,
        nodes: Nodes,
        i: usize,
    ) -> Option<ParseReturn> {
        Some(ParseReturn(Node::Nodes(nodes), i))
    }

    pub fn parse(&self) -> markdown::Nodes {
        if let Some(ParseReturn(Node::Nodes(nodes), _)) = self.parse_recursively(
            0,
            ParagraphParser::no_extra_end_condition,
            ParagraphParser::no_extra_wrap,
            ParagraphFlags::new(),
        ) {
            nodes
        } else {
            Vec::new()
        }
    }
}

struct CodeBlock {
    lang: Option<String>,
    lines: Vec<String>,
}

struct Callout {
    kind: Option<String>,
    lines: Vec<String>,
}

enum CurrentItem {
    CodeBlock(CodeBlock),
    Callout(Callout),
}

pub struct MarkdownParser {
    lines: Vec<String>,
}

impl MarkdownParser {
    pub fn for_string<S: ToString>(string: S) -> MarkdownParser {
        let lines = string
            .to_string()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        MarkdownParser { lines }
    }

    fn parse_thematic_break(&self, line: &str) -> Option<markdown::Block> {
        if line.chars().all(|c| c == '-' || c == '_') && line.len() >= 3 {
            Some(markdown::Block::ThematicBreak)
        } else {
            None
        }
    }

    fn parse_heading(&self, line: &str) -> Option<markdown::Heading> {
        match count_leading_chars(line, '#') {
            0 => None,
            level => {
                let text: String = line.chars().skip(level).skip_while(|c| *c == ' ').collect();
                Some(markdown::Heading(
                    level as u8,
                    ParagraphParser::for_string(text).parse(),
                ))
            }
        }
    }

    fn parse_bullet_point(&self, line: &str) -> Option<markdown::BulletPoint> {
        // Check for multiple spaces followed by a bullet point character (either - or *)

        let leading_spaces = count_leading_chars(line, ' ');

        let first_char = line.chars().skip(leading_spaces).next();

        if first_char == Some('-') || first_char == Some('*') {
            let text: String = line
                .chars()
                .skip(leading_spaces + 1)
                .skip_while(|c| *c == ' ')
                .collect();

            // If the text starts with a task indicator (a [ at position 0 and a ] at position 2), parse it as a task
            let (status, text) = split_task_string(&text);

            Some(markdown::BulletPoint(
                status,
                ParagraphParser::for_string(text).parse(),
            ))
        } else {
            None
        }
    }

    fn parse_paragraph(&self, text: &str) -> markdown::Nodes {
        ParagraphParser::for_string(text).parse()
    }

    fn try_parse_line(&self, line: &str) -> Option<markdown::Block> {
        if let Some(thematic_break) = self.parse_thematic_break(line) {
            Some(thematic_break)
        } else if let Some(heading) = self.parse_heading(line) {
            Some(markdown::Block::Heading(heading))
        } else if let Some(bullet_point) = self.parse_bullet_point(line) {
            Some(markdown::Block::BulletPoint(bullet_point))
        } else {
            None
        }
    }

    pub fn pre_parse_lines(&self, lines: &Vec<String>) -> Vec<PreParsed> {
        let mut pre_parsed = Vec::new();
        let mut current_block = Vec::new();
        let mut current_item = None;

        for line in lines {
            if let Some(CurrentItem::CodeBlock(cb)) = &mut current_item {
                if line == "```" {
                    pre_parsed.push(PreParsed::Parsed(Box::new(markdown::Block::Code(
                        cb.lang.clone(),
                        cb.lines.join("\n"),
                    ))));
                    current_item = None;
                } else {
                    cb.lines.push(line.clone());
                }
            } else if let Some(CurrentItem::Callout(callout)) = &mut current_item {
                if line.starts_with(">") {
                    callout.lines.push(line.chars().skip(2).collect());
                } else {
                    pre_parsed.push(PreParsed::Parsed(Box::new(markdown::Block::Callout(
                        callout.kind.clone(),
                        self.parse_lines(&callout.lines),
                    ))));
                    current_item = None;
                    current_block.push(line.clone());
                }
            } else if line.starts_with("```") {
                if !current_block.is_empty() {
                    pre_parsed.push(PreParsed::Unparsed(Box::new(current_block)));
                    current_block = Vec::new();
                }
                current_item = Some(CurrentItem::CodeBlock(CodeBlock {
                    lang: if line.len() > 3 {
                        Some(line.chars().skip(3).collect())
                    } else {
                        None
                    },
                    lines: Vec::new(),
                }));
            } else if line.starts_with(">") {
                if !current_block.is_empty() {
                    pre_parsed.push(PreParsed::Unparsed(Box::new(current_block)));
                    current_block = Vec::new();
                }

                let line: String = line.chars().skip(2).collect();

                // If the line is "[!kind] text" (ignore leading whitespaces), parse the kind and start a new callout

                let (lines, kind) = if line.starts_with("[!") {
                    let kind_end = line.find(']').unwrap();
                    let kind = line.chars().skip(2).take(kind_end - 2).collect();
                    // let line: String = line.chars().skip(kind_end + 2).collect();

                    (vec![], Some(kind))
                } else {
                    (vec![line], None)
                };

                current_item = Some(CurrentItem::Callout(Callout { kind, lines }));
            } else if line.starts_with(" - ") || line.starts_with(" * ") {
                if !current_block.is_empty() {
                    pre_parsed.push(PreParsed::Unparsed(Box::new(current_block)));
                    current_block = Vec::new();
                }
                pre_parsed.push(PreParsed::Parsed(Box::new(markdown::Block::BulletPoint(
                    self.parse_bullet_point(line).unwrap(),
                ))));
            } else if let Some(block) = self.try_parse_line(line) {
                if !current_block.is_empty() {
                    pre_parsed.push(PreParsed::Unparsed(Box::new(current_block)));
                    current_block = Vec::new();
                }
                pre_parsed.push(PreParsed::Parsed(Box::new(block)));
            } else {
                current_block.push(line.clone());
            }
        }

        if !current_block.is_empty() {
            pre_parsed.push(PreParsed::Unparsed(Box::new(current_block)));
        } else if let Some(CurrentItem::CodeBlock(cb)) = current_item {
            pre_parsed.push(PreParsed::Parsed(Box::new(markdown::Block::Code(
                cb.lang,
                cb.lines.join("\n"),
            ))));
        } else if let Some(CurrentItem::Callout(callout)) = current_item {
            pre_parsed.push(PreParsed::Parsed(Box::new(markdown::Block::Callout(
                callout.kind,
                self.parse_lines(&callout.lines),
            ))));
        }

        pre_parsed
    }

    fn parse_lines(&self, lines: &Vec<String>) -> markdown::Blocks {
        self.pre_parse_lines(lines)
            .into_iter()
            .map(|pre_parsed| match pre_parsed {
                PreParsed::Parsed(block) => *block,
                PreParsed::Unparsed(lines) => {
                    markdown::Block::Nodes(self.parse_paragraph(&lines.join("\n")))
                }
            })
            .collect()
    }

    pub fn parse(&self) -> Result<markdown::Document, ParseError> {
        let blocks = self.parse_lines(&self.lines);

        Ok(markdown::Document { blocks })
    }
}

pub fn parse_document(text: String) -> Result<markdown::Document, ParseError> {
    MarkdownParser::for_string(text).parse()
}

///
/// Parse a text snippet into a list of nodes.
///
pub fn parse_text_snippet<S: ToString>(content: S) -> Result<super::Nodes, ()> {
    let parser = ParagraphParser::for_string(content);
    Ok(parser.parse())
}
