use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

use crate::config;

#[component]
pub fn Header() -> impl IntoView {
    let (is_menu_open, set_menu_open) = signal(false);

    let toggle_menu = move |_| set_menu_open.update(|v| *v = !*v);
    let close_menu = move |_| set_menu_open.set(false);

    view! {
        <header class="bg-[rgba(15,15,15,0.92)] border-b border-[#262626] sticky top-0 z-50 backdrop-blur-sm">
            <div class="container-responsive">
                <div class="flex justify-between items-center h-16 min-h-16 relative">
                    // Brand
                    <A
                        href="/"
                        attr:class="font-display text-lg font-bold text-white hover:text-[#1ea6d5] transition-colors duration-200"
                    >
                        {config::SITE_NAME}
                    </A>

                    // Desktop navigation
                    <nav class="hidden md:flex space-x-1">
                        <NavLink href="/" label="Home" exact=true />
                        <NavLink href="/blog" label="Blog" />
                    </nav>

                    // Mobile menu button
                    <div class="md:hidden">
                        <button
                            type="button"
                            on:click=toggle_menu
                            class="text-[#9ca3af] hover:text-white focus:outline-none focus:ring-2 focus:ring-[#1ea6d5] focus:ring-offset-2 focus:ring-offset-[#0f0f0f] p-2 rounded-md transition-colors duration-200"
                            aria-expanded=move || is_menu_open.get().to_string()
                            aria-label="Toggle navigation menu"
                        >
                            <span class="sr-only">{ "Open main menu" }</span>
                            {move || if is_menu_open.get() {
                                view! {
                                    <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                }.into_any()
                            } else {
                                view! {
                                    <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                                    </svg>
                                }.into_any()
                            }}
                        </button>
                    </div>
                </div>
            </div>

            // Mobile menu
            <div class=move || format!(
                "md:hidden transition-all duration-300 overflow-hidden {}",
                if is_menu_open.get() { "max-h-64 opacity-100" } else { "max-h-0 opacity-0" }
            )>
                <div
                    class="px-4 pt-2 pb-4 space-y-1 bg-[#0f0f0f] border-t border-[#262626]"
                    on:click=close_menu
                >
                    <NavLink href="/" label="Home" exact=true mobile=true />
                    <NavLink href="/blog" label="Blog" mobile=true />
                </div>
            </div>
        </header>
    }
}

#[component]
fn NavLink(
    href: &'static str,
    label: &'static str,
    #[prop(optional)] exact: bool,
    #[prop(optional)] mobile: bool,
) -> impl IntoView {
    let location = use_location();
    let is_active = move || {
        let path = location.pathname.get();
        if exact {
            path == href
        } else {
            path == href || path.starts_with(&format!("{href}/"))
        }
    };

    let class = move || {
        let base = if mobile {
            "block px-4 py-3 text-sm font-medium rounded-md transition-colors duration-200"
        } else {
            "px-4 py-2 font-mono text-sm rounded-md transition-all duration-200"
        };
        let state = if is_active() {
            "text-[#1ea6d5] bg-[#1a1a1a]"
        } else {
            "text-[#9ca3af] hover:text-white hover:bg-[#1a1a1a]"
        };
        format!("{base} {state}")
    };

    view! {
        <A href=href attr:class=class>{label}</A>
    }
}
