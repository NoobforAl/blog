#[derive(PartialEq)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub updated: Option<&'static str>,
    pub excerpt: &'static str,
    pub tags: &'static [&'static str],
    pub content: &'static str,
    pub meta_description: &'static str,
    pub meta_keywords: &'static [&'static str],
    pub og_image: &'static str,
    pub twitter_image: &'static str,
    pub author: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/posts_gen.rs"));

pub fn all() -> &'static [Post] {
    POSTS
}

pub fn by_slug(slug: &str) -> Option<&'static Post> {
    POSTS.iter().find(|p| p.slug == slug)
}

pub fn related(current_slug: &str, limit: usize) -> Vec<&'static Post> {
    let current = by_slug(current_slug);

    let current = match current {
        Some(c) if !c.tags.is_empty() => c,
        _ => {
            return POSTS
                .iter()
                .filter(|p| p.slug != current_slug)
                .take(limit)
                .collect();
        }
    };

    let mut related: Vec<&'static Post> = POSTS
        .iter()
        .filter(|p| p.slug != current_slug)
        .filter(|p| p.tags.iter().any(|t| current.tags.contains(t)))
        .take(limit)
        .collect();

    if related.len() < limit {
        let extra: Vec<&'static Post> = POSTS
            .iter()
            .filter(|p| p.slug != current_slug)
            .filter(|p| !related.iter().any(|r| r.slug == p.slug))
            .take(limit - related.len())
            .collect();
        related.extend(extra);
    }

    related
}
