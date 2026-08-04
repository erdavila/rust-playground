use markdown::mdast::{Node, ReferenceKind};

pub(crate) fn lines_from_node(node: Node, max_line_len: usize) -> Vec<String> {
    let mut lines = Vec::new();

    match node {
        Node::Code(code) => {
            if let Some(meta) = code.meta {
                unimplemented!("{meta:?}");
            }
            lines.push(format!("```{}", code.lang.as_deref().unwrap_or("")));
            lines.extend(code.value.lines().map(ToOwned::to_owned));
            lines.push("```".to_string());
        }
        Node::Definition(def) => {
            let label = def.label.unwrap_or_else(|| unimplemented!());
            lines.push(format!("[{label}]: {}", def.url));
        }
        Node::Heading(h) => {
            let mut content = "#".repeat(h.depth.into());
            content.push(' ');
            content.push_children(h.children);
            lines.push(content);
        }
        Node::List(list) => {
            for (i, child) in list.children.into_iter().enumerate() {
                let Node::ListItem(li) = &child else {
                    unimplemented!("{child:?}");
                };
                if let Some(checked) = li.checked {
                    unimplemented!("{checked:?}");
                }

                let marker = if list.ordered {
                    format!("{}.", i + 1)
                } else {
                    "-".to_string()
                };
                let marker_len = marker.chars().count();

                let item_lines = lines_from_node(child, max_line_len - (marker_len + 1))
                    .into_iter()
                    .enumerate()
                    .map(|(i, item_line)| {
                        if i == 0 {
                            format!("{marker} {item_line}")
                        } else {
                            format!("{:marker_len$} {item_line}", "")
                        }
                    });
                lines.extend(item_lines);
            }
        }
        Node::ListItem(li) => {
            lines.extend(lines_from_children(li.children, max_line_len));
        }
        Node::Paragraph(p) => {
            let mut content = String::new();
            content.push_children(p.children);
            lines.extend(split_lines(&content, max_line_len));
        }
        Node::Root(root) => {
            lines.extend(lines_from_children(root.children, max_line_len));
        }
        _ => unimplemented!("{node:#?}"),
    }

    lines
}

fn lines_from_children(children: Vec<Node>, max_line_len: usize) -> Vec<String> {
    let mut lines = Vec::new();

    let mut prev_was_definition = None;
    for child in children {
        let curr_is_definition = matches!(child, Node::Definition(_));
        if prev_was_definition.is_some()
            && !(prev_was_definition == Some(true) && curr_is_definition)
        {
            lines.push(String::new());
        }

        lines.extend(lines_from_node(child, max_line_len));

        prev_was_definition = Some(curr_is_definition);
    }

    lines
}

fn split_lines(mut content: &str, max_line_len: usize) -> Vec<String> {
    let mut lines = Vec::new();

    content = content.trim();
    while !content.is_empty() {
        let mut should_break = false;
        let mut break_idx = None;
        for (idx, c) in content.char_indices() {
            if c == ' ' {
                break_idx = Some(idx);
            }

            if idx >= max_line_len {
                should_break = true;
                if break_idx.is_some() {
                    break;
                }
            }
        }

        if should_break && let Some(break_idx) = break_idx {
            // Line too long and there is a space char.
            let (line, rest) = content.split_at(break_idx);
            lines.push(line.trim().to_string());
            content = rest.trim();
        } else {
            lines.push(content.to_string());
            content = "";
        }
    }

    lines
}

trait StringExt {
    fn push_children(&mut self, children: Vec<Node>);
    fn push_chars(&mut self, s: &str);
}

impl StringExt for String {
    fn push_children(&mut self, children: Vec<Node>) {
        for child in children {
            match child {
                Node::Emphasis(emph) => {
                    self.push('_');
                    self.push_children(emph.children);
                    self.push('_');
                }
                Node::Html(html) => {
                    self.push_chars(&html.value);
                }
                Node::InlineCode(inl_code) => {
                    self.push('`');
                    self.push_chars(&inl_code.value);
                    self.push('`');
                }
                Node::Link(link) => {
                    self.push('[');
                    self.push_children(link.children);
                    self.push_str("](");
                    self.push_str(&link.url);
                    self.push(')');
                }
                Node::LinkReference(link_ref) => {
                    if link_ref.reference_kind != ReferenceKind::Shortcut {
                        unimplemented!("{:?}", link_ref.reference_kind);
                    }
                    self.push('[');
                    self.push_children(link_ref.children);
                    self.push(']');
                }
                Node::Strong(strong) => {
                    self.push_str("**");
                    self.push_children(strong.children);
                    self.push_str("**");
                }
                Node::Text(text) => self.push_chars(&text.value),
                _ => unimplemented!("{child:#?}"),
            }
        }
    }

    fn push_chars(&mut self, s: &str) {
        self.extend(s.chars().map(|c| if c == '\n' { ' ' } else { c }));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn split_lines() {
        use super::split_lines;

        assert_eq!(split_lines("abc", 10), ["abc"]);
        assert_eq!(split_lines("abcde fghi", 10), ["abcde fghi"]);
        assert_eq!(split_lines("abcde fghij", 10), ["abcde", "fghij"]);
        assert_eq!(split_lines("abcde fgh ijkl", 10), ["abcde fgh", "ijkl"]);
        assert_eq!(split_lines("abcde fghi jkl", 10), ["abcde fghi", "jkl"]);
        assert_eq!(split_lines("abcde fghij kl", 10), ["abcde", "fghij kl"]);
        assert_eq!(split_lines("abcdefghi jklmno", 10), ["abcdefghi", "jklmno"]);
        assert_eq!(split_lines("abcdefghij klmno", 10), ["abcdefghij", "klmno"]);
        assert_eq!(split_lines("abcdefghijk lmno", 10), ["abcdefghijk", "lmno"]);
        assert_eq!(split_lines("abc defghi jk", 10), ["abc defghi", "jk"]);
        assert_eq!(split_lines("abc defghij kl", 10), ["abc", "defghij kl"]);
        assert_eq!(
            split_lines("abc defghijk lm", 10),
            ["abc", "defghijk", "lm"]
        );
        assert_eq!(
            split_lines("abc defghijkl mn", 10),
            ["abc", "defghijkl", "mn"]
        );
        assert_eq!(
            split_lines("abc defghijklm no", 10),
            ["abc", "defghijklm", "no"]
        );
        assert_eq!(
            split_lines("abc defghijklmn op", 10),
            ["abc", "defghijklmn", "op"]
        );

        // Non-wrapping spaces are preserved
        assert_eq!(split_lines("abc    def", 10), ["abc    def"]);

        // Wrapping spaces are collapsed
        assert_eq!(split_lines("abc    defg", 10), ["abc", "defg"]);
    }
}
