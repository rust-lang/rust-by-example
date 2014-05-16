// bind Text to std::strbuf::StrBuf
use Text = std::strbuf::StrBuf;

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function")
        }
    }
}

fn main() {
    // Text can be used instead of StrBuf
    let mut text = Text::new();
    text.push_str("Hello");
    text.push_char(' ');
    text.push_str("World!");

    println!("{}", text);

    if true {
        // if a new name is omitted, the item name gets bound to the path
        use deeply::nested::function;

        function();

        // `use` bindings have a local scope, in this case the `function`
        // binding can only be used inside this if block
    }

    // Error: unresolved name `function`
    //function();
}
