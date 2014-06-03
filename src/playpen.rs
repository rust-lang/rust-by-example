pub fn escape(code: &str) -> String {
    let mut s = String::new();

    for c in code.chars() {
        match c {
            c@'!' => s.push_char(c),
            c@'.' => s.push_char(c),
            c@'0'..'9' => s.push_char(c),
            c@'A'..'Z' => s.push_char(c),
            c@'a'..'z' => s.push_char(c),
            c => s.push_str(format!("%{:02X}", c as u32).as_slice()),
        }
    }

    s
}

pub fn link(source: &str) -> String {
    format!("http://playtest.rust-lang.org/?code={}&run=1", escape(source))
}
