use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-[#0f0f0f] border-t border-[#262626] mt-auto">
            <div class="container-responsive py-6">
                <div class="text-center space-y-2">
                    <p class="text-[#6b7280] text-xs sm:text-sm">
                        <span class="font-mono">{ "built with Rust + Leptos" }</span>
                        <span class="text-[#374151]">{ " · " }</span>
                        <a
                            href="/feed.xml"
                            class="font-mono text-[#9ca3af] hover:text-[#1ea6d5] transition-colors duration-200"
                        >
                            { "rss" }
                        </a>
                    </p>
                    <p class="text-[#6b7280] text-xs font-mono">
                        { "An XOR can scare the devil — even 0x20 can." }
                    </p>
                </div>
            </div>
        </footer>
    }
}
