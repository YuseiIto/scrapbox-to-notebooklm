use crate::models::Page;
use regex::Regex;
use std::borrow::Cow;
use std::sync::LazyLock;

static LINK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[(.*?)\]").unwrap());

pub fn parse_page(page: &Page) -> String {
    let mut output = String::new();
    let mut inside_code_block = false;

    for (i, line_obj) in page.lines.iter().enumerate() {
        let raw_text = line_obj.text();

        // コードブロックの処理
        if inside_code_block {
            if raw_text.starts_with(' ') || raw_text.starts_with('\t') {
                // インデントを1つ落として出力
                let content = if !raw_text.is_empty() {
                    &raw_text[1..]
                } else {
                    raw_text
                };
                output.push_str(content);
                output.push('\n');
                continue;
            } else {
                inside_code_block = false;
                output.push_str("```\n\n");
            }
        }

        // コードブロックの開始判定
        if raw_text.starts_with("code:") {
            inside_code_block = true;
            let lang = raw_text.trim_start_matches("code:").trim();
            output.push_str(&format!("```{lang}\n"));
            continue;
        }

        let (indent, content) = parse_indent(raw_text);

        // 最初の行はタイトルとして扱う
        if i == 0 {
            output.push_str(&format!("# {content}\n\n"));
            continue;
        }

        if content.is_empty() && indent == 0 {
            output.push('\n');
            continue;
        }

        let formatted = format_content(content);

        // インデントがある場合はリスト、ない場合は段落
        if indent > 0 {
            let prefix = "  ".repeat(indent - 1) + "- ";
            output.push_str(&format!("{prefix}{formatted}\n"));
        } else {
            output.push_str(&formatted);
            output.push_str("\n\n");
        }
    }

    if inside_code_block {
        output.push_str("```\n");
    }

    output
}

fn parse_indent(text: &str) -> (usize, &str) {
    let trimmed = text.trim_start_matches([' ', '\t']);
    let indent = text.len() - trimmed.len();
    (indent, trimmed)
}

fn format_content(text: &str) -> Cow<str> {
    LINK_RE.replace_all(text, |caps: &regex::Captures| {
        let inner = &caps[1];

        // 装飾 (太字, 斜体, 打消)
        if let Some(stripped) = inner.strip_prefix("* ") {
            return format!("**{stripped}**");
        }
        if let Some(stripped) = inner.strip_prefix("/ ") {
            return format!("*{stripped}*");
        }
        if let Some(stripped) = inner.strip_prefix("- ") {
            return format!("~~{stripped}~~");
        }

        // 外部リンク (http:// または https://)
        if inner.contains("http://") || inner.contains("https://") {
            let parts: Vec<&str> = inner.split_whitespace().collect();
            if parts.len() == 1 {
                return format!("[{}]({})", parts[0], parts[0]);
            } else {
                // [text url] か [url text] かを判定
                let url_idx = parts
                    .iter()
                    .position(|p| p.starts_with("http://") || p.starts_with("https://"));
                if let Some(idx) = url_idx {
                    let url = parts[idx];
                    let text_parts: Vec<&str> = parts
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != idx)
                        .map(|(_, s)| *s)
                        .collect();
                    let text = text_parts.join(" ");
                    return format!("[{text}]({url})");
                }
            }
        }

        // 内部リンク
        format!("[[{inner}]]")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Line;

    #[test]
    fn test_format_content() {
        assert_eq!(format_content("[Link]"), "[[Link]]");
        assert_eq!(format_content("[* Bold]"), "**Bold**");
        assert_eq!(
            format_content("[https://example.com]"),
            "[https://example.com](https://example.com)"
        );
        assert_eq!(
            format_content("[Example https://example.com]"),
            "[Example](https://example.com)"
        );
    }

    #[test]
    fn test_parse_page() {
        let page = Page {
            title: "Test".to_string(),
            lines: vec![
                Line::String("Test Title".to_string()),
                Line::String("Line 1".to_string()),
                Line::String(" Line 2 (indented)".to_string()),
                Line::String("code:rust".to_string()),
                Line::String(" fn main() {}".to_string()),
            ],
        };
        let result = parse_page(&page);
        assert!(result.contains("# Test Title"));
        assert!(result.contains("- Line 2 (indented)"));
        assert!(result.contains("```rust"));
        assert!(result.contains("fn main() {}"));
    }
}
