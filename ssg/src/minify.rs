/// A conservative CSS minifier: strips comments, collapses whitespace runs,
/// and removes whitespace hugging `{ } : ; ,`. Deliberately leaves spacing
/// inside `calc()` and similar value expressions alone (it never touches
/// anything not immediately adjacent to those five characters), so it can't
/// break `calc(50vw - var(--x))`-style expressions the way an aggressive
/// minifier can.
pub fn css(input: &str) -> String {
    let mut no_comments = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
            i += 2;
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i += 2;
            continue;
        }
        no_comments.push(chars[i]);
        i += 1;
    }

    // Collapse whitespace runs to a single space.
    let mut collapsed = String::with_capacity(no_comments.len());
    let mut prev_ws = false;
    for c in no_comments.chars() {
        if c.is_whitespace() {
            if !prev_ws {
                collapsed.push(' ');
            }
            prev_ws = true;
        } else {
            collapsed.push(c);
            prev_ws = false;
        }
    }

    // Remove spaces hugging structural characters, and drop the trailing
    // semicolon before a closing brace.
    let mut out = String::with_capacity(collapsed.len());
    let cs: Vec<char> = collapsed.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c == ' ' {
            let prev = out.chars().last();
            let next = cs.get(i + 1).copied();
            let hugs = |ch: Option<char>| matches!(ch, Some('{') | Some('}') | Some(':') | Some(';') | Some(','));
            if hugs(prev) || hugs(next) {
                i += 1;
                continue;
            }
        }
        if c == ';' && cs[i + 1..].iter().find(|c| !c.is_whitespace()) == Some(&'}') {
            i += 1;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out.trim().to_string()
}
