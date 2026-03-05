fn main() {
    let raw_html = page::html();
    let minified_html: String =
        minify_html::minify(raw_html.as_bytes(), &minify_html::Cfg::default())
            .try_into()
            .expect("minifying returned invalid UTF-8");
    println!("{}", minified_html);
}
