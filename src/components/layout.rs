use leptos::prelude::*;

use super::footer::Footer;
use super::header::Header;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col overflow-x-hidden">
            <Header />
            <main class="flex-1 w-full">
                <div class="w-full">
                    {children()}
                </div>
            </main>
            <Footer />
        </div>
    }
}
