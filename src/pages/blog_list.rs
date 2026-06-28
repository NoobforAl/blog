use leptos::prelude::*;
use leptos_router::components::A;
use serde_json::json;

use crate::{date, meta, posts};

#[component]
pub fn BlogList() -> impl IntoView {
    Effect::new(move |_| {
        meta::apply(&meta::PageMeta {
            title: "Blog | x20",
            description: "Read the latest posts about programming, web development, and technology — tutorials, insights, and notes.",
            keywords: "blog, programming, web development, tutorials, technology, coding",
            path: "/blog",
            ..Default::default()
        });
        meta::scroll_to_top();
    });

    let posts = posts::all();
    let base_url = meta::site_url();

    let json_ld = json!({
        "@context": "https://schema.org",
        "@type": "Blog",
        "name": "x20",
        "description": "A blog about programming, web development, and technology.",
        "url": format!("{}/blog", base_url),
        "blogPost": posts.iter().map(|post| json!({
            "@type": "BlogPosting",
            "headline": post.title,
            "description": if post.excerpt.is_empty() { post.title } else { post.excerpt },
            "url": format!("{}/blog/{}", base_url, post.slug),
            "datePublished": post.date,
            "dateModified": post.updated.unwrap_or(post.date),
        })).collect::<Vec<_>>(),
    });

    view! {
        <script type="application/ld+json">{ json_ld.to_string() }</script>
        <div class="container-responsive py-8 sm:py-12">
            <div class="max-w-7xl mx-auto px-4">
                <div class="mb-8 sm:mb-12">
                    <h1 class="font-display text-3xl sm:text-4xl md:text-5xl font-bold text-white">
                        { "Writing" }
                    </h1>
                    <p class="text-[#9ca3af] mt-3 text-base sm:text-lg">
                        { "Articles and notes on software and the web." }
                    </p>
                </div>
                {if posts.is_empty() {
                    view! {
                        <div class="text-center py-16 sm:py-20 bg-[#141414] rounded-xl border border-[#262626]">
                            <p class="text-[#9ca3af] text-lg mb-2">{ "No posts yet." }</p>
                            <p class="text-[#6b7280] text-sm">{ "Check back soon for new content." }</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                            {posts.iter().map(post_card).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

fn post_card(post: &'static posts::Post) -> impl IntoView {
    let href = format!("/blog/{}", post.slug);
    view! {
        <article class="glow-card bg-[#141414] border border-[#262626] rounded-xl p-6 group flex flex-col">
            <div class="mb-3 space-y-1">
                <div class="font-mono text-xs text-[#6b7280]">
                    { date::short(post.date) }
                </div>
                {post.updated.map(|updated| view! {
                    <div class="font-mono text-xs text-[#9ca3af]">
                        { format!("updated {}", date::short(updated)) }
                    </div>
                })}
            </div>

            <h2 class="text-xl font-semibold mb-3 text-white group-hover:text-[#1ea6d5] transition-colors duration-200">
                <A href=href.clone() attr:class="hover:underline">
                    { post.title }
                </A>
            </h2>

            {(!post.excerpt.is_empty()).then(|| view! {
                <p class="text-[#9ca3af] text-sm mb-4 leading-relaxed flex-grow">
                    { post.excerpt }
                </p>
            })}

            {(!post.tags.is_empty()).then(|| view! {
                <div class="flex flex-wrap gap-2 mb-4">
                    {post.tags.iter().take(3).map(|tag| view! {
                        <span class="px-2.5 py-1 bg-[#0f0f0f] text-[#9ca3af] text-xs rounded-md border border-[#262626] hover:border-[#374151] transition-colors">
                            { *tag }
                        </span>
                    }).collect::<Vec<_>>()}
                    {(post.tags.len() > 3).then(|| view! {
                        <span class="px-2.5 py-1 bg-[#0f0f0f] text-[#6b7280] text-xs rounded-md border border-[#262626]">
                            { format!("+{}", post.tags.len() - 3) }
                        </span>
                    })}
                </div>
            })}

            <div class="flex items-center justify-start pt-4 border-t border-[#262626] mt-auto">
                <A
                    href=href
                    attr:class="text-[#1ea6d5] hover:text-[#1ea9d8] text-sm font-medium transition-colors duration-200 inline-flex items-center gap-1"
                >
                    { "Read more" }
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                    </svg>
                </A>
            </div>
        </article>
    }
}
