use std::env;
use std::fs;
use std::path::Path;

struct PostMeta {
    slug: String,
    title: String,
    date: String,
    updated: String,
    excerpt: String,
    tags: Vec<String>,
    content: String,
    meta_description: String,
    meta_keywords: Vec<String>,
    og_image: String,
    twitter_image: String,
    author: String,
}

fn yaml_str(v: &serde_yaml::Value, key: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string()
}

fn yaml_vec(v: &serde_yaml::Value, key: &str) -> Vec<String> {
    v.get(key)
        .and_then(|x| x.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|i| i.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

fn parse_post(path: &Path) -> Option<PostMeta> {
    let slug = path.file_stem()?.to_str()?.to_string();
    let raw = fs::read_to_string(path).ok()?;
    let raw = raw.replace("\r\n", "\n");

    let rest = raw.strip_prefix("---\n")?;
    let end = rest.find("\n---")?;
    let front = &rest[..end];
    let mut content = &rest[end + 4..];
    if let Some(stripped) = content.strip_prefix('\n') {
        content = stripped;
    }

    let fm: serde_yaml::Value = serde_yaml::from_str(front).unwrap_or_else(|e| {
        panic!("invalid frontmatter in {}: {}", path.display(), e);
    });

    let title = {
        let t = yaml_str(&fm, "title");
        if t.is_empty() {
            "Untitled".to_string()
        } else {
            t
        }
    };
    let excerpt = yaml_str(&fm, "excerpt");
    let tags = yaml_vec(&fm, "tags");

    let meta_description = {
        let d = yaml_str(&fm, "metaDescription");
        if d.is_empty() {
            excerpt.clone()
        } else {
            d
        }
    };
    let meta_keywords = {
        let k = yaml_vec(&fm, "metaKeywords");
        if k.is_empty() {
            tags.clone()
        } else {
            k
        }
    };

    Some(PostMeta {
        slug,
        title,
        date: yaml_str(&fm, "date"),
        updated: yaml_str(&fm, "updated"),
        excerpt,
        tags,
        content: content.to_string(),
        meta_description,
        meta_keywords,
        og_image: yaml_str(&fm, "ogImage"),
        twitter_image: yaml_str(&fm, "twitterImage"),
        author: yaml_str(&fm, "author"),
    })
}

fn str_slice_literal(items: &[String]) -> String {
    let parts: Vec<String> = items.iter().map(|s| format!("{:?}", s)).collect();
    format!("&[{}]", parts.join(", "))
}

fn generate_posts_rs(posts: &[PostMeta], out_dir: &str) {
    let mut code = String::from("pub static POSTS: &[Post] = &[\n");
    for p in posts {
        code.push_str(&format!(
            "    Post {{\n        slug: {slug:?},\n        title: {title:?},\n        date: {date:?},\n        updated: {updated},\n        excerpt: {excerpt:?},\n        tags: {tags},\n        content: {content:?},\n        meta_description: {meta_description:?},\n        meta_keywords: {meta_keywords},\n        og_image: {og_image:?},\n        twitter_image: {twitter_image:?},\n        author: {author:?},\n    }},\n",
            slug = p.slug,
            title = p.title,
            date = p.date,
            updated = if p.updated.is_empty() {
                "None".to_string()
            } else {
                format!("Some({:?})", p.updated)
            },
            excerpt = p.excerpt,
            tags = str_slice_literal(&p.tags),
            content = p.content,
            meta_description = p.meta_description,
            meta_keywords = str_slice_literal(&p.meta_keywords),
            og_image = p.og_image,
            twitter_image = p.twitter_image,
            author = p.author,
        ));
    }
    code.push_str("];\n");
    fs::write(Path::new(out_dir).join("posts_gen.rs"), code).unwrap();
}

fn generate_sitemap(posts: &[PostMeta], site_url: &str) {
    let latest = posts
        .iter()
        .map(|p| {
            if p.updated.is_empty() {
                p.date.as_str()
            } else {
                p.updated.as_str()
            }
        })
        .max()
        .unwrap_or("");

    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    for (route, priority) in [("", "1.0"), ("/blog", "0.8")] {
        xml.push_str("  <url>\n");
        xml.push_str(&format!("    <loc>{}{}</loc>\n", site_url, route));
        if !latest.is_empty() {
            xml.push_str(&format!("    <lastmod>{}</lastmod>\n", latest));
        }
        xml.push_str("    <changefreq>daily</changefreq>\n");
        xml.push_str(&format!("    <priority>{}</priority>\n", priority));
        xml.push_str("  </url>\n");
    }
    for p in posts {
        let lastmod = if p.updated.is_empty() {
            &p.date
        } else {
            &p.updated
        };
        xml.push_str("  <url>\n");
        xml.push_str(&format!("    <loc>{}/blog/{}</loc>\n", site_url, p.slug));
        if !lastmod.is_empty() {
            xml.push_str(&format!("    <lastmod>{}</lastmod>\n", lastmod));
        }
        xml.push_str("    <changefreq>weekly</changefreq>\n");
        xml.push_str("    <priority>0.7</priority>\n");
        xml.push_str("  </url>\n");
    }
    xml.push_str("</urlset>\n");
    fs::write("static/sitemap.xml", xml).unwrap();
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// "2024-01-10" -> "10 Jan 2024 00:00:00 GMT" (RFC 822, as required by RSS 2.0)
fn rfc822_date(date: &str) -> Option<String> {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mut parts = date.trim().splitn(3, '-');
    let year = parts.next()?;
    let month: usize = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    Some(format!(
        "{:02} {} {} 00:00:00 GMT",
        day,
        MONTHS[month - 1],
        year
    ))
}

fn generate_rss(posts: &[PostMeta], site_url: &str) {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\">\n");
    xml.push_str("  <channel>\n");
    xml.push_str("    <title>x20</title>\n");
    xml.push_str(&format!("    <link>{}</link>\n", site_url));
    xml.push_str("    <description>A developer blog sharing knowledge and insights about programming, web development, and technology.</description>\n");
    xml.push_str("    <language>en-us</language>\n");
    xml.push_str(&format!(
        "    <atom:link href=\"{}/feed.xml\" rel=\"self\" type=\"application/rss+xml\"/>\n",
        site_url
    ));
    if let Some(last_build) = posts.first().and_then(|p| {
        rfc822_date(if p.updated.is_empty() {
            &p.date
        } else {
            &p.updated
        })
    }) {
        xml.push_str(&format!(
            "    <lastBuildDate>{}</lastBuildDate>\n",
            last_build
        ));
    }
    for p in posts {
        let url = format!("{}/blog/{}", site_url, p.slug);
        xml.push_str("    <item>\n");
        xml.push_str(&format!("      <title>{}</title>\n", xml_escape(&p.title)));
        xml.push_str(&format!("      <link>{}</link>\n", url));
        xml.push_str(&format!(
            "      <guid isPermaLink=\"true\">{}</guid>\n",
            url
        ));
        if let Some(pub_date) = rfc822_date(&p.date) {
            xml.push_str(&format!("      <pubDate>{}</pubDate>\n", pub_date));
        }
        if !p.excerpt.is_empty() {
            xml.push_str(&format!(
                "      <description>{}</description>\n",
                xml_escape(&p.excerpt)
            ));
        }
        if !p.tags.is_empty() {
            for tag in &p.tags {
                xml.push_str(&format!("      <category>{}</category>\n", xml_escape(tag)));
            }
        }
        xml.push_str("    </item>\n");
    }
    xml.push_str("  </channel>\n</rss>\n");
    fs::write("static/feed.xml", xml).unwrap();
}

fn generate_robots(site_url: &str) {
    let robots = format!(
        "User-agent: *\nAllow: /\n\nSitemap: {}/sitemap.xml\n",
        site_url
    );
    fs::write("static/robots.txt", robots).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=content/blog");
    println!("cargo:rerun-if-env-changed=SITE_URL");

    let out_dir = env::var("OUT_DIR").unwrap();
    let site_url = env::var("SITE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let mut posts: Vec<PostMeta> = fs::read_dir("content/blog")
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().map(|x| x == "mdx").unwrap_or(false))
                .filter_map(|p| parse_post(&p))
                .collect()
        })
        .unwrap_or_default();

    posts.sort_by(|a, b| b.date.cmp(&a.date));

    generate_posts_rs(&posts, &out_dir);
    fs::create_dir_all("static").unwrap();
    generate_sitemap(&posts, &site_url);
    generate_robots(&site_url);
    generate_rss(&posts, &site_url);
}
