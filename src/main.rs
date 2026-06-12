mod components;
mod date;
mod markdown;
mod meta;
mod pages;
mod posts;
mod profile;

use yew::prelude::*;
use yew_router::prelude::*;

use components::layout::Layout;
use pages::blog_list::BlogList;
use pages::blog_post::BlogPost;
use pages::home::Home;
use pages::not_found::NotFound;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/blog/:slug")]
    BlogPost { slug: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Blog => html! { <BlogList /> },
        Route::BlogPost { slug } => html! { <BlogPost {slug} /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
