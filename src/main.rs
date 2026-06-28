mod components;
mod config;
mod date;
mod markdown;
mod meta;
mod pages;
mod posts;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use components::layout::Layout;
use pages::blog_list::BlogList;
use pages::blog_post::BlogPost;
use pages::home::Home;
use pages::not_found::NotFound;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Layout>
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/blog") view=BlogList />
                    <Route path=path!("/blog/:slug") view=BlogPost />
                </Routes>
            </Layout>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
