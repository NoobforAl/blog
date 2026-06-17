use yew::prelude::*;
use yew_router::prelude::*;

use crate::{date, meta, posts, profile, Route};

#[function_component(Home)]
pub fn home() -> Html {
    use_effect_with((), |_| {
        meta::apply(&meta::PageMeta {
            title: &format!("{} — {} | x20", profile::NAME, profile::ROLE),
            description: "A developer blog sharing knowledge and insights. I started this blog to deepen my knowledge and slow things down in a world moving at a frightening pace.",
            keywords: "blog, developer, programming, web development, technology, software development",
            path: "/",
            ..Default::default()
        });
        meta::scroll_to_top();
    });

    let latest_posts = posts::all();

    html! {
        <>
            // ---- Hero: $ whoami ----
            <div class="hero-bg">
                <div class="container-responsive relative z-10 pt-16 sm:pt-24 md:pt-32 pb-16 sm:pb-24">
                    <div class="max-w-4xl">
                        <p class="prompt-label rise rise-1">{ "whoami" }</p>
                        <h1 class="rise rise-2 font-display font-bold text-white leading-none tracking-tight text-5xl sm:text-7xl md:text-8xl mt-4 mb-6">
                            { profile::NAME }
                            <span class="blink-cursor" aria-hidden="true"></span>
                        </h1>
                        <p class="rise rise-3 font-mono text-sm sm:text-base text-[#1ea6d5] mb-4">
                            { profile::ROLE }
                        </p>
                        <p class="rise rise-4 text-[#9ca3af] text-base sm:text-lg leading-relaxed max-w-2xl mb-10">
                            { profile::TAGLINE }
                        </p>
                        <div class="rise rise-5 flex flex-wrap items-center gap-3">
                            <Link<Route>
                                to={Route::Blog}
                                classes="px-5 py-2.5 bg-[#1ea6d5] text-[#0f0f0f] font-mono text-sm font-semibold rounded-md hover:bg-[#1ea9d8] transition-colors duration-200"
                            >
                                { "cd /blog" }
                            </Link<Route>>
                            <a
                                href={profile::GITHUB_URL}
                                target="_blank"
                                rel="noopener noreferrer"
                                class="px-5 py-2.5 bg-transparent text-[#9ca3af] font-mono text-sm rounded-md hover:text-white transition-colors duration-200"
                            >
                                { "github ↗" }
                            </a>
                        </div>
                    </div>
                </div>
            </div>

            <div class="container-responsive pb-20 sm:pb-28 space-y-20 sm:space-y-28">

                // ---- About: $ cat about.md ----
                <section class="max-w-3xl">
                    <p class="prompt-label mb-4">{ "cat about.md" }</p>
                    <div class="terminal-card">
                        <div class="terminal-card-bar">
                            <span class="t-dot"></span>
                            <span class="t-dot"></span>
                            <span class="t-dot"></span>
                            <span class="font-mono text-xs text-[#6b7280] ml-2">{ "about.md" }</span>
                        </div>
                        <div class="p-6 sm:p-8 space-y-4">
                            { for profile::ABOUT.iter().map(|paragraph| html! {
                                <p class="text-[#9ca3af] text-base sm:text-lg leading-relaxed">
                                    { *paragraph }
                                </p>
                            }) }
                        </div>
                    </div>
                </section>

                // ---- Skills: $ ls skills/ ----
                <section>
                    <p class="prompt-label mb-6">{ "ls skills/" }</p>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 sm:gap-6">
                        { for profile::SKILLS.iter().map(|group| html! {
                            <div key={group.label} class="glow-card bg-[#141414] border border-[#262626] rounded-xl p-5 sm:p-6">
                                <h3 class="font-mono text-xs text-[#1ea6d5] mb-4">
                                    { format!("{}/", group.label) }
                                </h3>
                                <div class="flex flex-wrap gap-2">
                                    { for group.items.iter().map(|item| html! {
                                        <span
                                            key={*item}
                                            class="px-2.5 py-1 bg-[#0f0f0f] text-[#9ca3af] text-xs font-mono rounded border border-[#262626]"
                                        >
                                            { *item }
                                        </span>
                                    }) }
                                </div>
                            </div>
                        }) }
                    </div>
                </section>

                // ---- Latest Posts ----
                <section>
                    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between mb-6 sm:mb-8 gap-3 sm:gap-4">
                        <div>
                            <p class="prompt-label mb-2">{ "tail -n 3 blog.log" }</p>
                            <h2 class="font-display text-2xl sm:text-3xl md:text-4xl font-bold text-white">
                                { "Latest Posts" }
                            </h2>
                        </div>
                        <Link<Route>
                            to={Route::Blog}
                            classes="font-mono text-[#1ea6d5] hover:text-[#1ea9d8] transition-colors duration-200 text-sm self-start sm:self-center"
                        >
                            { "view all →" }
                        </Link<Route>>
                    </div>

                    if latest_posts.is_empty() {
                        <div class="text-center py-12 sm:py-16 bg-[#141414] rounded-xl border border-[#262626]">
                            <p class="text-[#9ca3af] text-base sm:text-lg mb-2">
                                { "No blog posts yet." }
                            </p>
                            <p class="text-[#6b7280] text-sm">
                                { "Check back soon for new content!" }
                            </p>
                        </div>
                    } else {
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
                            { for latest_posts.iter().take(3).map(post_card) }
                        </div>
                    }
                </section>

                // ---- Contact ----
                <section class="text-center py-8 sm:py-12">
                    <p class="font-mono text-sm text-[#6b7280] mb-4">
                        <span class="text-[#10b981]">{ "$ " }</span>
                        { "echo \"let's build something quiet and solid\"" }
                    </p>
                    <a
                        href={profile::GITHUB_URL}
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-block px-8 py-3 bg-[#141414] text-white font-mono text-sm border border-[#374151] rounded-lg hover:border-[#1ea6d5] hover:text-[#1ea6d5] transition-colors duration-200"
                    >
                        { "find me on GitHub ↗" }
                    </a>
                </section>
            </div>
        </>
    }
}

fn post_card(post: &'static posts::Post) -> Html {
    html! {
        <article
            key={post.slug}
            class="glow-card bg-[#141414] border border-[#262626] rounded-xl p-4 sm:p-6 group flex flex-col h-full"
        >
            <div class="mb-3 space-y-1">
                <time class="font-mono text-xs text-[#6b7280] block">
                    { date::short(post.date) }
                </time>
                if let Some(updated) = post.updated {
                    <time class="font-mono text-xs text-[#9ca3af] block">
                        { format!("updated {}", date::short(updated)) }
                    </time>
                }
            </div>

            <h3 class="text-lg sm:text-xl font-semibold mb-2 sm:mb-3 text-white group-hover:text-[#1ea6d5] transition-colors duration-200">
                <Link<Route> to={Route::BlogPost { slug: post.slug.to_string() }}>
                    { post.title }
                </Link<Route>>
            </h3>

            if !post.excerpt.is_empty() {
                <p class="text-[#9ca3af] text-sm mb-3 sm:mb-4 line-clamp-3 flex-grow">
                    { post.excerpt }
                </p>
            }

            if !post.tags.is_empty() {
                <div class="flex flex-wrap gap-2 mb-3 sm:mb-4">
                    { for post.tags.iter().take(3).map(|tag| html! {
                        <span
                            key={*tag}
                            class="px-2 py-1 bg-[#0f0f0f] text-[#9ca3af] text-xs font-mono rounded border border-[#262626]"
                        >
                            { *tag }
                        </span>
                    }) }
                </div>
            }

            <div class="flex items-center justify-start mt-auto pt-3 sm:pt-4 border-t border-[#262626]">
                <Link<Route>
                    to={Route::BlogPost { slug: post.slug.to_string() }}
                    classes="font-mono text-[#1ea6d5] hover:text-[#1ea9d8] text-sm transition-colors duration-200"
                >
                    { "read more →" }
                </Link<Route>>
            </div>
        </article>
    }
}
