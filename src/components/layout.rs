use yew::prelude::*;

use super::footer::Footer;
use super::header::Header;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="min-h-screen flex flex-col hacker-bg overflow-x-hidden">
            <Header />
            <main class="flex-1 w-full">
                <div class="w-full">
                    { props.children.clone() }
                </div>
            </main>
            <Footer />
        </div>
    }
}
