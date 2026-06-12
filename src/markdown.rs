use pulldown_cmark::{html, Options, Parser};
use yew::{AttrValue, Html};

pub fn render(markdown: &str) -> Html {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut out = String::new();
    html::push_html(&mut out, parser);

    Html::from_html_unchecked(AttrValue::from(out))
}
