pub fn editor(source: &str) -> String {
    format!("<div id=\"active-code\">
<button class=\"btn btn-primary\" type=\"button\" id=\"run-code\">Run</button>
<button class=\"btn btn-primary\" type=\"button\" id=\"reset-code\">Reset</button>
<div id=\"editor\"><pre>{}</pre></div>
<div id=\"result\"></div>
</div>", escape(source))
}

fn escape(source: &str) -> String {
    let mut s = String::new();

    for chr in source.trim().chars() {
        match chr {
            '$' => s.push_str("&#36;"),
            '*' => s.push_str("&#42;"),
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            '\\' => s.push_str("&#92;"),
            '_' => s.push_str("&#95;"),
            '`' => s.push_str("&#96;"),
            chr => s.push(chr),
        }
    }

    s
}
