use serde_json::json;
use yew::prelude::*;
use yew_router::prelude::*;

use super::not_found::NotFound;
use crate::{date, markdown, meta, posts, Route};

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub slug: String,
}

#[function_component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    let post = posts::by_slug(&props.slug);

    use_effect_with(props.slug.clone(), move |slug| {
        if let Some(post) = post {
            let description = if !post.meta_description.is_empty() {
                post.meta_description.to_string()
            } else if !post.excerpt.is_empty() {
                post.excerpt.to_string()
            } else {
                format!("Read {}", post.title)
            };
            let keywords = if !post.meta_keywords.is_empty() {
                post.meta_keywords.join(", ")
            } else {
                post.tags.join(", ")
            };
            let og_image = if post.og_image.is_empty() {
                None
            } else if post.og_image.starts_with("http") {
                Some(post.og_image.to_string())
            } else {
                Some(format!("{}{}", meta::site_url(), post.og_image))
            };
            meta::apply(&meta::PageMeta {
                title: &format!("{} | x20", post.title),
                description: &description,
                keywords: &keywords,
                path: &format!("/blog/{}", slug),
                og_type: "article",
                og_image,
                published_time: Some(post.date),
                modified_time: Some(post.updated.unwrap_or(post.date)),
                robots: "index, follow",
            });
        }
        meta::scroll_to_top();
        meta::highlight_code_blocks();
    });

    let Some(post) = post else {
        return html! { <NotFound /> };
    };

    let related_posts = posts::related(&props.slug, 3);
    let base_url = meta::site_url();
    let post_url = format!("{}/blog/{}", base_url, post.slug);

    let description = if !post.meta_description.is_empty() {
        post.meta_description
    } else if !post.excerpt.is_empty() {
        post.excerpt
    } else {
        post.title
    };
    let author = if post.author.is_empty() {
        "Developer"
    } else {
        post.author
    };
    let og_image = if post.og_image.is_empty() {
        format!("{}/og-image.png", base_url)
    } else if post.og_image.starts_with("http") {
        post.og_image.to_string()
    } else {
        format!("{}{}", base_url, post.og_image)
    };
    let keywords = if !post.meta_keywords.is_empty() {
        post.meta_keywords
    } else {
        post.tags
    };

    let json_ld = json!({
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "headline": post.title,
        "description": description,
        "image": og_image,
        "datePublished": post.date,
        "dateModified": post.updated.unwrap_or(post.date),
        "author": {
            "@type": "Person",
            "name": author,
        },
        "publisher": {
            "@type": "Organization",
            "name": "x20",
        },
        "mainEntityOfPage": {
            "@type": "WebPage",
            "@id": post_url,
        },
        "keywords": keywords.join(", "),
    });

    html! {
        <>
            <script type="application/ld+json">{ json_ld.to_string() }</script>
            <div class="container-responsive py-8 sm:py-12">
                <div class="max-w-4xl mx-auto px-4">
                    // Back to blog
                    <div class="mb-6 sm:mb-8">
                        <Link<Route>
                            to={Route::Blog}
                            classes="inline-flex items-center text-[#9ca3af] hover:text-white transition-colors duration-200 text-sm"
                        >
                            <svg class="w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                            </svg>
                            { "Back to Blog" }
                        </Link<Route>>
                    </div>

                    // Article Header
                    <article class="mb-12 sm:mb-16">
                        <header class="mb-8 sm:mb-12">
                            <h1 class="font-display text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold mb-6 text-white leading-tight">
                                { post.title }
                            </h1>

                            <div class="flex flex-col gap-4 mb-6">
                                <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                                    <div class="space-y-1">
                                        <time class="text-sm text-[#9ca3af] block">
                                            { format!("Published: {}", date::long(post.date)) }
                                        </time>
                                        if let Some(updated) = post.updated {
                                            <time class="text-sm text-[#6b7280] block">
                                                { format!("Updated: {}", date::long(updated)) }
                                            </time>
                                        }
                                    </div>

                                    if !post.tags.is_empty() {
                                        <div class="flex flex-wrap gap-2">
                                            { for post.tags.iter().map(|tag| html! {
                                                <span
                                                    key={*tag}
                                                    class="px-3 py-1 bg-[#1a1a1a] text-[#9ca3af] text-sm rounded-md border border-[#374151]"
                                                >
                                                    { *tag }
                                                </span>
                                            }) }
                                        </div>
                                    }
                                </div>
                            </div>
                        </header>

                        // Article Content
                        <div class="bg-[#1a1a1a] border border-[#374151] rounded-lg p-6 sm:p-8 lg:p-12">
                            <div class="markdown-body max-w-none">
                                { markdown::render(post.content) }
                            </div>
                        </div>
                    </article>

                    // Related Posts
                    if !related_posts.is_empty() {
                        <section class="mt-16 sm:mt-20 pt-8 sm:pt-12 border-t border-[#374151]">
                            <h2 class="text-2xl sm:text-3xl font-bold mb-8 text-white">
                                { "Related Posts" }
                            </h2>
                            <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
                                { for related_posts.iter().map(|related| related_card(related)) }
                            </div>
                        </section>
                    }

                    // Navigation
                    <div class="mt-12 sm:mt-16 pt-8 sm:pt-12 border-t border-[#374151]">
                        <div class="flex justify-center">
                            <Link<Route>
                                to={Route::Blog}
                                classes="px-6 py-3 bg-[#1a1a1a] text-white border border-[#374151] rounded-lg font-medium hover:bg-[#262626] hover:border-[#4b5563] transition-all duration-200"
                            >
                                { "View All Posts" }
                            </Link<Route>>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}

fn related_card(post: &'static posts::Post) -> Html {
    html! {
        <article
            key={post.slug}
            class="bg-[#1a1a1a] border border-[#374151] rounded-lg p-6 hover:border-[#4b5563] hover:shadow-lg transition-all duration-200 group"
        >
            <h3 class="text-lg font-semibold mb-3 text-white group-hover:text-[#1ea6d5] transition-colors duration-200">
                <Link<Route> to={Route::BlogPost { slug: post.slug.to_string() }}>
                    { post.title }
                </Link<Route>>
            </h3>

            if !post.excerpt.is_empty() {
                <p class="text-[#9ca3af] text-sm mb-4 line-clamp-2">
                    { post.excerpt }
                </p>
            }

            <div class="flex items-center justify-between pt-4 border-t border-[#374151]">
                <time class="text-xs text-[#6b7280]">
                    { date::short(post.date) }
                </time>
                <Link<Route>
                    to={Route::BlogPost { slug: post.slug.to_string() }}
                    classes="text-[#1ea6d5] hover:text-[#1ea9d8] text-sm font-medium transition-colors duration-200"
                >
                    { "Read →" }
                </Link<Route>>
            </div>
        </article>
    }
}
