use leptos::prelude::*;
use leptos_router::components::A;

use crate::meta;

#[component]
pub fn NotFound() -> impl IntoView {
    Effect::new(move |_| {
        meta::apply(&meta::PageMeta {
            title: "Page Not Found | x20",
            description: "The page you're looking for doesn't exist or has been moved.",
            path: "/404",
            robots: "noindex",
            ..Default::default()
        });
        meta::scroll_to_top();
    });

    view! {
        <div class="container-responsive py-16 sm:py-24">
            <div class="max-w-2xl mx-auto px-4 text-center">
                <h1 class="text-6xl sm:text-7xl font-bold text-[#1ea6d5] mb-6">{ "404" }</h1>
                <h2 class="text-2xl sm:text-3xl font-semibold text-white mb-4">
                    { "Page Not Found" }
                </h2>
                <p class="text-[#9ca3af] mb-8">
                    { "The page you're looking for doesn't exist or has been moved." }
                </p>
                <A
                    href="/"
                    attr:class="inline-block px-6 py-3 bg-[#1a1a1a] text-white border border-[#262626] rounded-lg font-medium hover:bg-[#262626] hover:border-[#1ea6d5] transition-all duration-200"
                >
                    { "Back to Home" }
                </A>
            </div>
        </div>
    }
}
