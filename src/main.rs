mod components;

use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use components::article_component::{ArticleComponent, ArticleComponentDetails};
use components::breaking_block_component::BreakingBlockComponent;
use components::helpers::codeur_facile_description_block;
use components::helpers::{facebook_link, fill_articles, github_link, twitter_link, youtube_link};
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

fn list_articles() -> Html {
    fill_articles().iter()
        .map(|(id, article)| {
            let id = *id;
            let path = "#".to_string() + &Route::Article{id}.to_path();
            html! { <a href={path} class="notUnderlinedLink"><ArticleComponent article_block = { article.clone() } /></a> }
        } ).collect()
}

fn home() -> Html {
    html! {
        <>
            <h1></h1>
            <div id="breakingBlock">
                <BreakingBlockComponent breaking_block = { codeur_facile_description_block() } />
            </div>
            <div id="blog">
                <h1 id="globalArticleTitle">{"Articles du blog: "}</h1>
                <div id="blogArticles">
                    { list_articles() }
                </div>
            </div>
        </>
    }
}

fn render_home_link() -> Html {
    html! {
        <>
            <div class="socialMediaBlockWrapper">
                <div class="socialMediaBlock" style="width: 50px">
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span class="fa fa-home"></span>
                </div>
            </div>
        </>
    }
}

fn footer() -> Html {
    html! {
        <footer><div>
            <a href={Route::Home.to_path()}>{ render_home_link() }</a>
            <SocialMediaBlockComponent social_media_block = { youtube_link() } />
            <SocialMediaBlockComponent social_media_block = { github_link() } />
            <SocialMediaBlockComponent social_media_block = { twitter_link() } />
            <SocialMediaBlockComponent social_media_block = { facebook_link() } />
        </div></footer>
    }
}

fn article(id_input: usize) -> Html {
    fill_articles()
        .iter()
        .filter(|(id_article, _)| id_input == **id_article)
        .map(|(_, article)| {
            html! {<ArticleComponentDetails article_block = { article.clone() } />}
        })
        .collect()
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
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
            {footer()}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
