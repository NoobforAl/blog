use wasm_bindgen::prelude::*;
use web_sys::Document;

#[wasm_bindgen]
extern "C" {
    /// Defined in index.html; runs highlight.js over un-highlighted code blocks.
    #[wasm_bindgen(js_name = highlightCodeBlocks, catch)]
    fn highlight_code_blocks_js() -> Result<(), JsValue>;
}

/// Per-page document metadata (replaces Next.js per-route Metadata for SEO).
pub struct PageMeta<'a> {
    pub title: &'a str,
    pub description: &'a str,
    /// Comma-separated keyword list.
    pub keywords: &'a str,
    /// Route path used for the canonical URL and og:url, e.g. "/blog/my-post".
    pub path: &'a str,
    /// "website" or "article".
    pub og_type: &'a str,
    pub og_image: Option<String>,
    pub published_time: Option<&'a str>,
    pub modified_time: Option<&'a str>,
    /// e.g. "index, follow" or "noindex".
    pub robots: &'a str,
}

impl Default for PageMeta<'_> {
    fn default() -> Self {
        PageMeta {
            title: "Developer Blog",
            description: "",
            keywords: "",
            path: "/",
            og_type: "website",
            og_image: None,
            published_time: None,
            modified_time: None,
            robots: "index, follow",
        }
    }
}

fn document() -> Option<Document> {
    web_sys::window().and_then(|w| w.document())
}

/// Update (or create) a meta tag identified by the given attribute selector.
fn upsert_meta(doc: &Document, attr: &str, key: &str, content: &str) {
    let selector = format!("meta[{}='{}']", attr, key);
    if let Ok(Some(el)) = doc.query_selector(&selector) {
        let _ = el.set_attribute("content", content);
    } else if let (Ok(el), Some(head)) = (doc.create_element("meta"), doc.head()) {
        let _ = el.set_attribute(attr, key);
        let _ = el.set_attribute("content", content);
        let _ = head.append_child(&el);
    }
}

fn remove_meta(doc: &Document, attr: &str, key: &str) {
    let selector = format!("meta[{}='{}']", attr, key);
    if let Ok(Some(el)) = doc.query_selector(&selector) {
        el.remove();
    }
}

fn upsert_meta_opt(doc: &Document, attr: &str, key: &str, content: Option<&str>) {
    match content {
        Some(content) => upsert_meta(doc, attr, key, content),
        None => remove_meta(doc, attr, key),
    }
}

fn set_canonical(doc: &Document, url: &str) {
    if let Ok(Some(el)) = doc.query_selector("link[rel='canonical']") {
        let _ = el.set_attribute("href", url);
    } else if let (Ok(el), Some(head)) = (doc.create_element("link"), doc.head()) {
        let _ = el.set_attribute("rel", "canonical");
        let _ = el.set_attribute("href", url);
        let _ = head.append_child(&el);
    }
}

/// Apply the full set of SEO tags for the current page.
pub fn apply(meta: &PageMeta) {
    let Some(doc) = document() else { return };

    doc.set_title(meta.title);

    let url = format!("{}{}", site_url(), meta.path);
    set_canonical(&doc, &url);

    upsert_meta(&doc, "name", "description", meta.description);
    upsert_meta(&doc, "name", "robots", meta.robots);
    if !meta.keywords.is_empty() {
        upsert_meta(&doc, "name", "keywords", meta.keywords);
    }

    upsert_meta(&doc, "property", "og:title", meta.title);
    upsert_meta(&doc, "property", "og:description", meta.description);
    upsert_meta(&doc, "property", "og:type", meta.og_type);
    upsert_meta(&doc, "property", "og:url", &url);
    upsert_meta_opt(&doc, "property", "og:image", meta.og_image.as_deref());
    upsert_meta_opt(
        &doc,
        "property",
        "article:published_time",
        meta.published_time,
    );
    upsert_meta_opt(
        &doc,
        "property",
        "article:modified_time",
        meta.modified_time,
    );

    upsert_meta(&doc, "name", "twitter:title", meta.title);
    upsert_meta(&doc, "name", "twitter:description", meta.description);
    upsert_meta_opt(&doc, "name", "twitter:image", meta.og_image.as_deref());
}

pub fn scroll_to_top() {
    if let Some(window) = web_sys::window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
}

/// Origin of the current page, used for canonical URLs and JSON-LD.
pub fn site_url() -> String {
    web_sys::window()
        .and_then(|w| w.location().origin().ok())
        .unwrap_or_else(|| "http://localhost:3000".to_string())
}

pub fn highlight_code_blocks() {
    let _ = highlight_code_blocks_js();
}
