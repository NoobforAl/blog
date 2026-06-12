use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-[#0f0f0f] border-t border-[#262626] mt-auto">
            <div class="container-responsive py-6">
                <div class="text-center space-y-2">
                    <p class="text-[#6b7280] text-xs sm:text-sm">
                        { "Created by " }
                        <a
                            href={crate::profile::GITHUB_URL}
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-[#9ca3af] hover:text-[#1ea6d5] transition-colors duration-200"
                        >
                            { "NoobforAl" }
                        </a>
                        <span class="text-[#374151]">{ " · " }</span>
                        <span class="font-mono">{ "built with Rust + Yew" }</span>
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
