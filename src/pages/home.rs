use leptos::prelude::*;
use leptos_router::components::A;

use crate::{config, date, meta, posts};

#[component]
pub fn Home() -> impl IntoView {
    Effect::new(move |_| {
        meta::apply(&meta::PageMeta {
            title: "x20",
            description: "A blog about programming, systems, and the layers beneath the web.",
            keywords: "blog, programming, software development, systems, web, technology",
            path: "/",
            ..Default::default()
        });
        meta::scroll_to_top();
    });

    let latest_posts = posts::all();

    view! {
        // ---- Hero ----
        <section class="page-hero">
            <div class="container-responsive">
                <div class="max-w-3xl">
                    <h1 class="rise rise-1 font-display font-bold text-white leading-tight tracking-tight text-4xl sm:text-6xl mb-4">
                        {config::SITE_NAME}
                    </h1>
                    <p class="rise rise-2 text-[#9ca3af] text-lg sm:text-xl leading-relaxed mb-8">
                        {config::SITE_TAGLINE}
                    </p>
                    <div class="rise rise-3">
                        <A
                            href="/blog"
                            attr:class="inline-block px-5 py-2.5 bg-[#1ea6d5] text-[#0f0f0f] text-sm font-semibold rounded-md hover:bg-[#1ea9d8] transition-colors duration-200"
                        >
                            { "Read the blog" }
                        </A>
                    </div>
                </div>
            </div>
        </section>

        // ---- Recent posts ----
        <div class="container-responsive py-16 sm:py-20">
            <section>
                <div class="flex items-center justify-between mb-8">
                    <h2 class="font-display text-2xl sm:text-3xl font-bold text-white">
                        { "Recent posts" }
                    </h2>
                    {(!latest_posts.is_empty()).then(|| view! {
                        <A
                            href="/blog"
                            attr:class="text-[#1ea6d5] hover:text-[#1ea9d8] transition-colors duration-200 text-sm"
                        >
                            { "View all →" }
                        </A>
                    })}
                </div>

                {if latest_posts.is_empty() {
                    view! {
                        <div class="text-center py-12 sm:py-16 bg-[#141414] rounded-xl border border-[#262626]">
                            <p class="text-[#9ca3af] text-base sm:text-lg mb-2">{ "No posts yet." }</p>
                            <p class="text-[#6b7280] text-sm">{ "Check back soon for new content." }</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
                            {latest_posts.iter().take(3).map(post_card).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }}
            </section>
        </div>
    }
}

fn post_card(post: &'static posts::Post) -> impl IntoView {
    let href = format!("/blog/{}", post.slug);
    view! {
        <article class="glow-card bg-[#141414] border border-[#262626] rounded-xl p-5 sm:p-6 group flex flex-col h-full">
            <div class="mb-3 space-y-1">
                <time class="font-mono text-xs text-[#6b7280] block">
                    { date::short(post.date) }
                </time>
                {post.updated.map(|updated| view! {
                    <time class="font-mono text-xs text-[#9ca3af] block">
                        { format!("updated {}", date::short(updated)) }
                    </time>
                })}
            </div>

            <h3 class="text-lg sm:text-xl font-semibold mb-2 sm:mb-3 text-white group-hover:text-[#1ea6d5] transition-colors duration-200">
                <A href=href.clone()>
                    { post.title }
                </A>
            </h3>

            {(!post.excerpt.is_empty()).then(|| view! {
                <p class="text-[#9ca3af] text-sm mb-3 sm:mb-4 line-clamp-3 flex-grow">
                    { post.excerpt }
                </p>
            })}

            {(!post.tags.is_empty()).then(|| view! {
                <div class="flex flex-wrap gap-2 mb-3 sm:mb-4">
                    {post.tags.iter().take(3).map(|tag| view! {
                        <span class="px-2 py-1 bg-[#0f0f0f] text-[#9ca3af] text-xs font-mono rounded border border-[#262626]">
                            { *tag }
                        </span>
                    }).collect::<Vec<_>>()}
                </div>
            })}

            <div class="flex items-center justify-start mt-auto pt-3 sm:pt-4 border-t border-[#262626]">
                <A
                    href=href
                    attr:class="text-[#1ea6d5] hover:text-[#1ea9d8] text-sm transition-colors duration-200"
                >
                    { "Read more →" }
                </A>
            </div>
        </article>
    }
}
