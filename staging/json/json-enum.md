### 49.10.1 `Json`

`serialize::json::Json` is an enum capable of containing all of the
JSON value types. It is Rust's way of representing JSON documents.

`Json` can be decoded from a JSON string using `seralize::json::from_str`:

<div id="active-code">
<button class="btn btn-primary" type="button" id="run-code">Run</button>
<button class="btn btn-primary" type="button" id="reset-code">Reset</button>
<div id="editor">extern crate serialize;

use serialize::json;

fn main() {
    for &json&#95;str in [
        // Valid JSON
        r#"{"name": "London", "lat": 51.507222, "lon": -0.1275}"#,

        // Not a valid JSON, will produce an error
        r#"{"name": "Prague" "lat": 50.082542, "lon": 14.425992}"#,
        // FIXME            ^ a comma is missing here
    ].iter() {
        // &#96;from&#95;str&#96; decodes JSON from a string and returns a &#96;Result&#96;
        // containing either a &#96;Json&#96; enum or an error
        match json::from&#95;str(json&#95;str) {
            Ok(json) =&gt; println!("json::from&#95;str(): {}", json),
            Err(err) =&gt; println!("json::from&#95;str(): {}", err),
        };
    }
}</div>
<div id="result"></div>
</div>
