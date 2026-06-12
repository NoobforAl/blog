use serde_json::json;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{date, meta, posts, Route};

#[function_component(BlogList)]
pub fn blog_list() -> Html {
    use_effect_with((), |_| {
        meta::apply(&meta::PageMeta {
            title: "Blog | Developer Blog",
            description: "Read the latest blog posts about programming, web development, and technology. Explore tutorials, insights, and knowledge sharing.",
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
        "name": "Developer Blog",
        "description": "A developer blog sharing knowledge and insights about programming, web development, and technology.",
        "url": format!("{}/blog", base_url),
        "blogPost": posts.iter().map(|post| json!({
            "@type": "BlogPosting",
            "headline": post.title,
            "description": if post.excerpt.is_empty() { post.title } else { post.excerpt },
            "url": format!("{}/blog/{}", base_url, post.slug),
            "datePublished": post.date,
            "dateModified": post.updated.unwrap_or(post.date),
            "author": {
                "@type": "Person",
                "name": "Developer",
            },
        })).collect::<Vec<_>>(),
    });

    html! {
        <>
            <script type="application/ld+json">{ json_ld.to_string() }</script>
            <div class="container-responsive py-8 sm:py-12">
                <div class="max-w-7xl mx-auto px-4">
                    <div class="mb-8 sm:mb-12">
                        <p class="prompt-label mb-2">{ "ls posts/ --sort=date" }</p>
                        <h1 class="font-display text-3xl sm:text-4xl md:text-5xl font-bold text-white">
                            { "Writing" }
                        </h1>
                    </div>
                    if posts.is_empty() {
                        <div class="text-center py-16 sm:py-20 bg-[#1a1a1a] rounded-lg border border-[#374151]">
                            <p class="text-[#9ca3af] text-lg mb-2">
                                { "No blog posts yet." }
                            </p>
                            <p class="text-[#6b7280] text-sm">
                                { "Check back soon for new content!" }
                            </p>
                        </div>
                    } else {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                            { for posts.iter().map(post_card) }
                        </div>
                    }
                </div>
            </div>
        </>
    }
}

fn post_card(post: &'static posts::Post) -> Html {
    html! {
        <article
            key={post.slug}
            class="glow-card bg-[#141414] border border-[#262626] rounded-xl p-6 group flex flex-col"
        >
            <div class="mb-3 space-y-1">
                <div class="font-mono text-xs text-[#6b7280]">
                    { date::short(post.date) }
                </div>
                if let Some(updated) = post.updated {
                    <div class="font-mono text-xs text-[#9ca3af]">
                        { format!("updated {}", date::short(updated)) }
                    </div>
                }
            </div>

            <h2 class="text-xl font-semibold mb-3 text-white group-hover:text-[#1ea6d5] transition-colors duration-200">
                <Link<Route>
                    to={Route::BlogPost { slug: post.slug.to_string() }}
                    classes="hover:underline"
                >
                    { post.title }
                </Link<Route>>
            </h2>

            if !post.excerpt.is_empty() {
                <p class="text-[#9ca3af] text-sm mb-4 leading-relaxed flex-grow">
                    { post.excerpt }
                </p>
            }

            if !post.tags.is_empty() {
                <div class="flex flex-wrap gap-2 mb-4">
                    { for post.tags.iter().take(3).map(|tag| html! {
                        <span
                            key={*tag}
                            class="px-2.5 py-1 bg-[#0f0f0f] text-[#9ca3af] text-xs rounded-md border border-[#374151] hover:border-[#4b5563] transition-colors"
                        >
                            { *tag }
                        </span>
                    }) }
                    if post.tags.len() > 3 {
                        <span class="px-2.5 py-1 bg-[#0f0f0f] text-[#6b7280] text-xs rounded-md border border-[#374151]">
                            { format!("+{}", post.tags.len() - 3) }
                        </span>
                    }
                </div>
            }

            <div class="flex items-center justify-start pt-4 border-t border-[#374151] mt-auto">
                <Link<Route>
                    to={Route::BlogPost { slug: post.slug.to_string() }}
                    classes="text-[#1ea6d5] hover:text-[#1ea9d8] text-sm font-medium transition-colors duration-200 inline-flex items-center gap-1"
                >
                    { "Read more" }
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                    </svg>
                </Link<Route>>
            </div>
        </article>
    }
}
