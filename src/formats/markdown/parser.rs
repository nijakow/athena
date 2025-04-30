use crate::formats::markdown;

use super::Node;

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

    fn parse_bold_from(&self, index: usize, flags: ParagraphFlags) -> Option<ParseLink> {
        let mut current = String::new();
        let mut i = index;

        while !self.at_end(i) {
            if let (true, new_i) = self.check_at(i, "**") {
                return Some(ParseLink::adjoin(
                    Node::Bold(Self::for_string(current).parse_with_flags(flags.with_bold())),
                    self.parse_from(new_i, flags),
                ));
            }

            current.push(self.at(i).unwrap());
            i += 1;
        }

        None
    }

    fn parse_italic_from(&self, index: usize, flags: ParagraphFlags) -> Option<ParseLink> {
        let mut current = String::new();
        let mut i = index;

        while !self.at_end(i) {
            if let (true, new_i) = self.check_at(i, "*") {
                return Some(ParseLink::adjoin(
                    Node::Italic(Self::for_string(current).parse_with_flags(flags.with_italic())),
                    self.parse_from(new_i, flags),
                ));
            }

            current.push(self.at(i).unwrap());
            i += 1;
        }

        None
    }

    fn parse_internal_link(
        &self,
        index: usize,
        flags: ParagraphFlags,
        embed: bool,
    ) -> Option<ParseLink> {
        let mut current = String::new();
        let mut i = index;

        while !self.at_end(i) {
            if let (true, new_i) = self.check_at(i, "]]") {
                let (target, title) = split_string_at_optional_pipe(&current);

                let title =
                    title.map(|title| Self::for_string(title).parse_with_flags(flags.with_link()));

                let link = markdown::Link {
                    kind: markdown::LinkKind::Internal,
                    target: markdown::LinkTarget::Zettel(target.to_string()),
                    title,
                };

                return Some(ParseLink::adjoin(
                    Node::Link { link, embed },
                    self.parse_from(new_i, flags),
                ));
            }

            current.push(self.at(i).unwrap());
            i += 1;
        }

        None
    }

    fn parse_external_link_part2(
        &self,
        index: usize,
        flags: ParagraphFlags,
        part1: String,
        embed: bool,
    ) -> Option<ParseLink> {
        let mut current = String::new();
        let mut i = index;

        while !self.at_end(i) {
            if let (true, new_i) = self.check_at(i, ")") {
                let link = markdown::Link {
                    kind: markdown::LinkKind::External,
                    target: markdown::LinkTarget::guess(current),
                    title: Some(Self::for_string(part1).parse_with_flags(flags.with_link())),
                };

                return Some(ParseLink::adjoin(
                    Node::Link { link, embed },
                    self.parse_from(new_i, flags),
                ));
            }

            current.push(self.at(i).unwrap());
            i += 1;
        }

        None
    }

    fn parse_external_link(
        &self,
        index: usize,
        flags: ParagraphFlags,
        embed: bool,
    ) -> Option<ParseLink> {
        let mut current = String::new();
        let mut i = index;

        // Parse until we reach a `](` combination - fail if we reach the end of the string

        while !self.at_end(i) {
            if let (true, new_i) = self.check_at(i, "](") {
                return self.parse_external_link_part2(new_i, flags, current, embed);
            }

            current.push(self.at(i).unwrap());
            i += 1;
        }

        None
    }

    fn parse_code(&self, index: usize, flags: ParagraphFlags) -> Option<ParseLink> {
        let mut current = String::new();
        let mut i = index;

        while !self.at_end(i) {
            if let (true, new_i) = self.check_at(i, "`") {
                return Some(ParseLink::adjoin(
                    Node::Code(current),
                    self.parse_from(new_i, flags),
                ));
            }

            current.push(self.at(i).unwrap());
            i += 1;
        }

        None
    }

    fn parse_tag(&self, index: usize, flags: ParagraphFlags) -> Option<ParseLink> {
        let mut current = String::new();
        let mut i = index;

        // Parse while we have a tag character (`is_tag_char`)

        while !self.at_end(i) {
            if !is_tag_char(self.at(i).unwrap()) {
                return Some(ParseLink::adjoin(
                    Node::Tag(current),
                    self.parse_from(i, flags),
                ));
            }

            current.push(self.at(i).unwrap());
            i += 1;
        }

        if current.is_empty() {
            None
        } else {
            Some(ParseLink::first(Node::Tag(current)))
        }
    }

    fn parse_from(&self, index: usize, flags: ParagraphFlags) -> ParseLink {
        let mut current = String::new();
        let mut i = index;

        while !self.at_end(i) {
            if !flags.bold {
                if let (true, new_i) = self.check_at(i, "**") {
                    if let Some(link) = self.parse_bold_from(new_i, flags) {
                        return ParseLink::adjoin(Node::Text(current), link);
                    }
                }
            }

            if !flags.italic {
                if let (true, new_i) = self.check_at(i, "*") {
                    if let Some(link) = self.parse_italic_from(new_i, flags) {
                        return ParseLink::adjoin(Node::Text(current), link);
                    }
                }
            }

            if !flags.link {
                if let (true, new_i) = self.check_at(i, "![[") {
                    if let Some(link) = self.parse_internal_link(new_i, flags, true) {
                        return ParseLink::adjoin(Node::Text(current), link);
                    }
                }

                if let (true, new_i) = self.check_at(i, "![") {
                    if let Some(link) = self.parse_external_link(new_i, flags, true) {
                        return ParseLink::adjoin(Node::Text(current), link);
                    }
                }

                if let (true, new_i) = self.check_at(i, "[[") {
                    if let Some(link) = self.parse_internal_link(new_i, flags, false) {
                        return ParseLink::adjoin(Node::Text(current), link);
                    }
                }

                if let (true, new_i) = self.check_at(i, "[") {
                    if let Some(link) = self.parse_external_link(new_i, flags, false) {
                        return ParseLink::adjoin(Node::Text(current), link);
                    }
                }

                if let (true, new_i) = self.check_at(i, "`") {
                    if let Some(link) = self.parse_code(new_i, flags) {
                        return ParseLink::adjoin(Node::Text(current), link);
                    }
                }

                if let (true, new_i) = self.check_at(i, "#") {
                    if let Some(link) = self.parse_tag(new_i, flags) {
                        return ParseLink::adjoin(Node::Text(current), link);
                    }
                }

                if let (true, new_i) = self.check_at(i, "\n") {
                    return ParseLink::adjoin(
                        Node::Text(current),
                        ParseLink::adjoin(Node::Newline, self.parse_from(new_i, flags)),
                    );
                }
            }

            current.push(self.at(i).unwrap());
            i += 1;
        }

        ParseLink::first(Node::Text(current))
    }

    pub fn parse_with_flags(&self, flags: ParagraphFlags) -> markdown::Nodes {
        let mut nodes = Vec::new();
        let mut link = self.parse_from(0, flags);

        loop {
            if let Node::Text(text) = *link.current {
                if !text.is_empty() {
                    nodes.push(Node::Text(text));
                }
            } else {
                nodes.push(*link.current);
            }

            if let Some(next) = link.next {
                link = *next;
            } else {
                break;
            }
        }

        nodes
    }

    pub fn parse(&self) -> markdown::Nodes {
        self.parse_with_flags(ParagraphFlags::new())
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

                current_item = Some(CurrentItem::Callout(Callout {
                    kind,
                    lines,
                }));
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
        self.pre_parse_lines(lines).into_iter().map(|pre_parsed| match pre_parsed {
            PreParsed::Parsed(block) => *block,
            PreParsed::Unparsed(lines) => {
                markdown::Block::Nodes(self.parse_paragraph(&lines.join("\n")))
            }
        }).collect()
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
