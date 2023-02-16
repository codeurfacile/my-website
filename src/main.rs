mod components;

use components::article_component::ArticleBlock;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use components::article_component::ArticleComponent;
use components::breaking_block_component::BreakingBlockComponent;
use components::helpers::codeur_facile_description_block;
use components::helpers::{
    facebook_link, github_link, home_link, introduction_rust_article, twitter_link, youtube_link,
};
use components::social_media_block_component::SocialMediaBlockComponent;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/article/:id")]
    Article { id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn home() -> Html {
    html! {
        <>
            <h1></h1>
            <div id="breakingBlock">
                <BreakingBlockComponent breaking_block = { codeur_facile_description_block() } />
            </div>
            <div id="blog">
                <h1>{"Articles du blog: "}</h1>
                <div id="blogArticles">
                    <ArticleComponent article_block = { introduction_rust_article(false) } />
                </div>
            </div>
        </>
    }
}

fn footer() -> Html {
    html! {
        <footer><div>
            <SocialMediaBlockComponent social_media_block = { home_link() } />
            <SocialMediaBlockComponent social_media_block = { youtube_link() } />
            <SocialMediaBlockComponent social_media_block = { github_link() } />
            <SocialMediaBlockComponent social_media_block = { twitter_link() } />
            <SocialMediaBlockComponent social_media_block = { facebook_link() } />
        </div></footer>
    }
}

fn no_article() -> Html {
    html! {
        <>
            <div>{"No article found for the given URL"}</div>
        </>
    }
}

fn render_article(article: ArticleBlock) -> Html {
    html! {
        <>
            <ArticleComponent article_block = { article } />
        </>
    }
}

fn article(id: usize) -> Html {
    match id {
        1 => render_article(introduction_rust_article(true)),
        _ => no_article(),
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => home(),
        Route::Article { id } => article(id),
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
            {footer()}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
