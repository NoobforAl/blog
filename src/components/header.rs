use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

const NAV_ITEMS: [(Route, &str); 2] = [(Route::Home, "Home"), (Route::Blog, "Blog")];

#[function_component(Header)]
pub fn header() -> Html {
    let route = use_route::<Route>();
    let is_menu_open = use_state(|| false);

    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_: MouseEvent| is_menu_open.set(!*is_menu_open))
    };

    let close_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_: MouseEvent| is_menu_open.set(false))
    };

    let desktop_nav = NAV_ITEMS.iter().map(|(item, label)| {
        let is_active = route.as_ref() == Some(item);
        let classes = format!(
            "px-4 py-2 font-mono text-sm rounded-md transition-all duration-200 {}",
            if is_active {
                "text-[#1ea6d5] bg-[#1a1a1a]"
            } else {
                "text-[#9ca3af] hover:text-white hover:bg-[#1a1a1a]"
            }
        );
        html! {
            <Link<Route> to={item.clone()} classes={Classes::from(classes)}>
                { *label }
            </Link<Route>>
        }
    });

    let mobile_nav = NAV_ITEMS.iter().map(|(item, label)| {
        let is_active = route.as_ref() == Some(item);
        let classes = format!(
            "block px-4 py-3 text-sm font-medium rounded-md transition-colors duration-200 {}",
            if is_active {
                "text-[#3b82f6] bg-[#1a1a1a]"
            } else {
                "text-[#9ca3af] hover:text-white hover:bg-[#1a1a1a]"
            }
        );
        html! {
            <Link<Route> to={item.clone()} classes={Classes::from(classes)}>
                { *label }
            </Link<Route>>
        }
    });

    html! {
        <header class="bg-[rgba(15,15,15,0.92)] border-b border-[#262626] sticky top-0 z-50 backdrop-blur-sm">
            <div class="container-responsive">
                <div class="flex justify-between items-center h-16 min-h-16 relative">
                    // Brand
                    <Link<Route>
                        to={Route::Home}
                        classes="font-mono text-sm text-[#e5e7eb] hover:text-white transition-colors duration-200"
                    >
                        <span class="text-[#10b981]">{ "~$ " }</span>
                        { "x20" }
                        <span class="blink-cursor" aria-hidden="true" style="width:0.45em;height:1em;"></span>
                    </Link<Route>>

                    // Navigation
                    <nav class="hidden md:flex space-x-1">
                        { for desktop_nav }
                    </nav>

                    // Mobile menu button
                    <div class="md:hidden">
                        <button
                            type="button"
                            onclick={toggle_menu}
                            class="text-[#9ca3af] hover:text-white focus:outline-none focus:ring-2 focus:ring-[#1ea6d5] focus:ring-offset-2 focus:ring-offset-[#0f0f0f] p-2 rounded-md transition-colors duration-200"
                            aria-expanded={is_menu_open.to_string()}
                            aria-label="Toggle navigation menu"
                        >
                            <span class="sr-only">{ "Open main menu" }</span>
                            if *is_menu_open {
                                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            } else {
                                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                                </svg>
                            }
                        </button>
                    </div>
                </div>
            </div>

            // Mobile menu
            <div class={format!(
                "md:hidden transition-all duration-300 overflow-hidden {}",
                if *is_menu_open { "max-h-64 opacity-100" } else { "max-h-0 opacity-0" }
            )}>
                <div
                    class="px-4 pt-2 pb-4 space-y-1 bg-[#0f0f0f] border-t border-[#374151]"
                    onclick={close_menu}
                >
                    { for mobile_nav }
                </div>
            </div>
        </header>
    }
}
