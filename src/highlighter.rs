use regex::Regex;
use scraper::{Html, Selector};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

fn _title_string(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// hightlight `code` and `pre` with syntect & scraper. Use lib scraper
/// to parse html DOM, find every code block. For each block,
/// we render it again using lib syntect to add code syntax highlighting.
pub fn highlight_html(html: &str) -> String {
    let mut html_new = html.to_string();
    let fragment = Html::parse_fragment(html);
    let select_pre = Selector::parse("pre").unwrap();
    let select_code = Selector::parse("code").unwrap();
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    for item in fragment.select(&select_pre) {
        for item_code in item.select(&select_code) {
            let code_html = item_code.html();
            // println!("code_html: {:?}", code_html);

            let re = Regex::new(r"code class=.language-([A-Za-z0-9]+).").unwrap();
            let lang = match re.captures(&code_html) {
                Some(cap) => {
                    cap[1].to_string()
                }
                None => String::new(),
            };

            let lang_titled = _title_string(&lang);
            let syntax = if syntax_set.find_syntax_by_extension(&lang).is_some() {
                syntax_set.find_syntax_by_extension(&lang)
            } else if syntax_set.find_syntax_by_name(&lang_titled).is_some() {
                syntax_set.find_syntax_by_name(&lang_titled)
            } else {
                None
            };
            if syntax.is_none() {
                continue;
            }

            let syntax = syntax.unwrap();

            let current_code = item_code.inner_html();
            // println!("code inner html (ori): {:?}", current_code);

            // comrak has encoded some code tokens, we need to decode it
            // before encoding them again with syntect.
            let raw_code = htmlescape::decode_html(&current_code).unwrap();
            // println!("code inner html (new): {:?}", raw_code);

            let output_html = highlighted_html_for_string(
                &raw_code, &syntax_set, &syntax,
                &ts.themes["Solarized (light)"]);
            // println!("output_html o: {:?}", output_html);

            // note here we replaced one newline char after <pre>
            let output_html = output_html.unwrap();
            let output_html = output_html.replacen("<pre style=\"background-color:#fdf6e3;\">\n", "<pre><code>", 1);
            let output_html = output_html.trim_end_matches("</pre>\n");
            let output_html = format!("{}</code></pre>\n", output_html);
            // println!("output_html n: {:?}", output_html);

            // handle some special chars, before replace plain code
            // to colored code.
            let content_old = current_code.replace("\"", "&quot;");
            // println!("content_old o: {:?}", content_old);

            let content_old = format!("<pre><code class=\"language-{}\">{}</code></pre>", lang, content_old);
            // println!("content_old n: {:?}", content_old);

            html_new = html_new.replace(&content_old, &output_html);
        }
    }
    html_new
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
