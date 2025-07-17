/// Indent a string by a given number of spaces for each line
pub fn indent(s: &str, indent: usize) -> String {
    let indent_str = " ".repeat(indent);
    s.lines()
        .map(|line| format!("{}{}", indent_str, line))
        .collect::<Vec<String>>()
        .join("\n")
}

/// Dedent a vector of strings by the minimum indentation
pub fn dedent<'a>(lines: &[&'a str]) -> Vec<&'a str> {
    let min_indent = lines
        .iter()
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap();

    // Learning note:
    // Equivalent to `&(*line)[min_indent..]`
    // indexing is method calling, so it will do derefer coercion
    // indexing a slice gives a slice
    // line[min_indent..] is of type str
    // &line[min_indent..] is of type &str
    lines.iter().map(|line| &line[min_indent..]).collect()
}

pub const BOX_DRAWING_CHARS: (&str, &str, &str, &str, &str, &str) =
    ("┌", "┐", "┘", "└", "─", "│");

/// Add a box around a string
pub fn add_box(s: &str) -> String {
    let (
        top_left,
        top_right,
        bottom_right,
        bottom_left,
        _horizontal,
        vertical,
    ) = BOX_DRAWING_CHARS;
    let lines = s.lines().collect::<Vec<&str>>();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap();
    let mut result = String::new();
    result.push_str(top_left);
    result.push_str(&"─".repeat(max_len));
    result.push_str(top_right);
    result.push_str("\n");
    for line in lines {
        result.push_str(format!("{}{}", vertical, line).as_str());
        let pad = " ".repeat(max_len - line.len());
        result.push_str(format!("{}{}", pad, vertical).as_str());
        result.push_str("\n");
    }
    result.push_str(bottom_left);
    result.push_str(&"─".repeat(max_len));
    result.push_str(bottom_right);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_box() {
        let s = "a\nbb\nc";
        let boxed = add_box(s);
        insta::assert_snapshot!(boxed, @r"
        ┌──┐
        │a │
        │bb│
        │c │
        └──┘
        ");
    }
}
